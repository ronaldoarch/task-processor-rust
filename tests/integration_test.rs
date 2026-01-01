use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn test_health_check() {
    let client = Client::new();
    let response = client
        .get(&format!("{}/api/health", BASE_URL))
        .send()
        .await
        .expect("Falha ao conectar ao servidor");

    assert!(response.status().is_success());
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "healthy");
}

#[tokio::test]
async fn test_create_and_list_tasks() {
    let client = Client::new();

    // Criar tarefa
    let create_response = client
        .post(&format!("{}/api/tasks", BASE_URL))
        .json(&json!({
            "name": "Tarefa de teste",
            "duration_ms": 1000,
            "priority": "high"
        }))
        .send()
        .await
        .expect("Falha ao criar tarefa");

    assert!(create_response.status().is_success());
    let task: serde_json::Value = create_response.json().await.unwrap();
    let task_id = task["id"].as_str().unwrap();

    // Listar tarefas
    let list_response = client
        .get(&format!("{}/api/tasks", BASE_URL))
        .send()
        .await
        .expect("Falha ao listar tarefas");

    assert!(list_response.status().is_success());
    let tasks: Vec<serde_json::Value> = list_response.json().await.unwrap();
    assert!(!tasks.is_empty());

    // Buscar tarefa específica
    let get_response = client
        .get(&format!("{}/api/tasks/{}", BASE_URL, task_id))
        .send()
        .await
        .expect("Falha ao buscar tarefa");

    assert!(get_response.status().is_success());
    let retrieved_task: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(retrieved_task["name"], "Tarefa de teste");
}

#[tokio::test]
async fn test_stats() {
    let client = Client::new();

    let response = client
        .get(&format!("{}/api/stats", BASE_URL))
        .send()
        .await
        .expect("Falha ao obter estatísticas");

    assert!(response.status().is_success());
    let stats: serde_json::Value = response.json().await.unwrap();
    assert!(stats["total_tasks"].is_number());
}

