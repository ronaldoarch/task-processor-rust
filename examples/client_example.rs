// Exemplo de cliente para testar a API
// Execute com: cargo run --example client_example

use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    println!("🚀 Criando tarefas de exemplo...\n");

    // Criar múltiplas tarefas com diferentes prioridades
    let tasks = vec![
        ("Processar dados críticos", 2000, "high"),
        ("Backup de arquivos", 3000, "medium"),
        ("Limpeza de cache", 1000, "low"),
        ("Geração de relatório", 4000, "medium"),
        ("Sincronização de dados", 2500, "high"),
    ];

    let mut task_ids = Vec::new();

    for (name, duration, priority) in tasks {
        let response = client
            .post(&format!("{}/api/tasks", BASE_URL))
            .json(&json!({
                "name": name,
                "duration_ms": duration,
                "priority": priority
            }))
            .send()
            .await?;

        if response.status().is_success() {
            let task: serde_json::Value = response.json().await?;
            let id = task["id"].as_str().unwrap();
            task_ids.push(id.to_string());
            println!("✅ Tarefa criada: {} (ID: {})", name, id);
        }
    }

    println!("\n📊 Obtendo estatísticas...");
    let stats_response = client
        .get(&format!("{}/api/stats", BASE_URL))
        .send()
        .await?;
    
    if stats_response.status().is_success() {
        let stats: serde_json::Value = stats_response.json().await?;
        println!("Total de tarefas: {}", stats["total_tasks"]);
        println!("Pendentes: {}", stats["pending"]);
        println!("Processando: {}", stats["processing"]);
    }

    println!("\n⏳ Aguardando processamento...");
    sleep(Duration::from_secs(5)).await;

    println!("\n📋 Listando todas as tarefas...");
    let list_response = client
        .get(&format!("{}/api/tasks", BASE_URL))
        .send()
        .await?;

    if list_response.status().is_success() {
        let tasks: Vec<serde_json::Value> = list_response.json().await?;
        for task in tasks {
            println!(
                "  - {}: {} ({})",
                task["name"].as_str().unwrap(),
                task["status"].as_str().unwrap(),
                task["id"].as_str().unwrap()
            );
        }
    }

    println!("\n✅ Exemplo concluído!");

    Ok(())
}

