use crate::models::{Stats, Task, TaskPriority, TaskStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info, warn};
use uuid::Uuid;

pub struct TaskProcessor {
    tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
    stats: Arc<Stats>,
    task_sender: broadcast::Sender<(Uuid, Task)>,
}

impl TaskProcessor {
    pub fn new() -> Self {
        let (task_sender, _) = broadcast::channel(1000);
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(Stats::new()),
            task_sender,
        }
    }

    pub async fn create_task(
        &self,
        name: String,
        duration_ms: u64,
        priority: TaskPriority,
    ) -> Task {
        let task = Task::new(name, duration_ms, priority);
        let task_id = task.id;

        // Adicionar à coleção
        let mut tasks = self.tasks.write().await;
        tasks.insert(task_id, task.clone());
        drop(tasks);

        // Atualizar estatísticas
        self.stats.increment_total();
        self.stats.increment_pending();

        // Notificar via broadcast
        let _ = self.task_sender.send((task_id, task.clone()));

        info!("✅ Tarefa criada: {} (ID: {})", task.name, task_id);
        task
    }

    pub async fn get_task(&self, id: Uuid) -> Option<Task> {
        let tasks = self.tasks.read().await;
        tasks.get(&id).cloned()
    }

    pub async fn list_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }

    pub async fn cancel_task(&self, id: Uuid) -> Result<(), String> {
        let mut tasks = self.tasks.write().await;
        
        if let Some(task) = tasks.get_mut(&id) {
            match task.status {
                TaskStatus::Pending => {
                    task.mark_as_cancelled();
                    self.stats.increment_cancelled();
                    self.stats.decrement_pending();
                    let _ = self.task_sender.send((id, task.clone()));
                    info!("🚫 Tarefa cancelada: {}", id);
                    Ok(())
                }
                TaskStatus::Processing => {
                    task.mark_as_cancelled();
                    self.stats.increment_cancelled();
                    let _ = self.task_sender.send((id, task.clone()));
                    warn!("⚠️ Tentativa de cancelar tarefa em processamento: {}", id);
                    Ok(())
                }
                _ => Err("Tarefa já foi finalizada".to_string()),
            }
        } else {
            Err("Tarefa não encontrada".to_string())
        }
    }

    pub fn get_stats(&self) -> crate::models::TaskStats {
        self.stats.get_stats()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(Uuid, Task)> {
        self.task_sender.subscribe()
    }

    pub async fn start_processing(&self) {
        info!("🔄 Iniciando processamento de tarefas...");
        
        loop {
            // Buscar tarefas pendentes
            let pending_tasks: Vec<(Uuid, Task)> = {
                let tasks = self.tasks.read().await;
                tasks
                    .iter()
                    .filter(|(_, task)| matches!(task.status, TaskStatus::Pending))
                    .map(|(id, task)| (*id, task.clone()))
                    .collect()
            };

            // Processar tarefas em paralelo
            let mut handles = Vec::new();
            
            for (id, task) in pending_tasks {
                let tasks = self.tasks.clone();
                let stats = self.stats.clone();
                let sender = self.task_sender.clone();

                // Ordenar por prioridade
                let priority_score = match task.priority {
                    TaskPriority::High => 3,
                    TaskPriority::Medium => 2,
                    TaskPriority::Low => 1,
                };

                let handle = tokio::spawn(async move {
                    // Marcar como processando
                    {
                        let mut tasks = tasks.write().await;
                        if let Some(t) = tasks.get_mut(&id) {
                            t.mark_as_processing();
                            stats.increment_processing();
                            let _ = sender.send((id, t.clone()));
                        }
                    }

                    info!("⚙️ Processando tarefa: {} ({}ms)", task.name, task.duration_ms);

                    // Simular processamento
                    tokio::time::sleep(tokio::time::Duration::from_millis(task.duration_ms)).await;

                    // Marcar como completada
                    {
                        let mut tasks = tasks.write().await;
                        if let Some(t) = tasks.get_mut(&id) {
                            if matches!(t.status, TaskStatus::Cancelled) {
                                return;
                            }
                            
                            // Calcular tempo de processamento
                            let processing_time = if let Some(started) = t.started_at {
                                (chrono::Utc::now() - started).num_milliseconds() as u64
                            } else {
                                t.duration_ms
                            };

                            // Simular falha ocasional (5% de chance)
                            if rand::random::<f64>() < 0.05 {
                                t.mark_as_failed("Erro aleatório durante processamento".to_string());
                                stats.increment_failed();
                                error!("❌ Tarefa falhou: {}", id);
                            } else {
                                t.mark_as_completed();
                                stats.increment_completed(processing_time);
                                info!("✅ Tarefa completada: {}", id);
                            }
                            
                            let _ = sender.send((id, t.clone()));
                        }
                    };
                });

                handles.push((priority_score, handle));
            }

            // Aguardar todas as tarefas (com prioridade)
            handles.sort_by(|a, b| b.0.cmp(&a.0));
            for (_, handle) in handles {
                let _ = handle.await;
            }

            // Aguardar um pouco antes da próxima iteração
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

impl Default for TaskProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_task() {
        let processor = TaskProcessor::new();
        let task = processor.create_task(
            "Test Task".to_string(),
            1000,
            TaskPriority::High,
        ).await;

        assert_eq!(task.name, "Test Task");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.priority, TaskPriority::High);

        let retrieved = processor.get_task(task.id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Task");
    }

    #[tokio::test]
    async fn test_list_tasks() {
        let processor = TaskProcessor::new();
        
        processor.create_task("Task 1".to_string(), 1000, TaskPriority::Low).await;
        processor.create_task("Task 2".to_string(), 2000, TaskPriority::Medium).await;
        
        let tasks = processor.list_tasks().await;
        assert_eq!(tasks.len(), 2);
    }

    #[tokio::test]
    async fn test_cancel_task() {
        let processor = TaskProcessor::new();
        let task = processor.create_task(
            "Task to cancel".to_string(),
            1000,
            TaskPriority::Low,
        ).await;

        let result = processor.cancel_task(task.id).await;
        assert!(result.is_ok());

        let cancelled_task = processor.get_task(task.id).await.unwrap();
        assert_eq!(cancelled_task.status, TaskStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_stats() {
        let processor = TaskProcessor::new();
        
        processor.create_task("Task 1".to_string(), 1000, TaskPriority::High).await;
        processor.create_task("Task 2".to_string(), 2000, TaskPriority::Medium).await;
        
        let stats = processor.get_stats();
        assert_eq!(stats.total_tasks, 2);
        assert_eq!(stats.pending, 2);
    }
}
