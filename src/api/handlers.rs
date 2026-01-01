use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

use crate::models::{CreateTaskRequest, Task, TaskStats};
use crate::AppState;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "task-processor",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<Json<Task>, (StatusCode, Json<serde_json::Value>)> {
    if request.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Nome da tarefa não pode estar vazio"
            })),
        ));
    }

    if request.duration_ms == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Duração deve ser maior que zero"
            })),
        ));
    }

    let task = state.processor.create_task(
        request.name,
        request.duration_ms,
        request.priority,
    ).await;

    Ok(Json(task))
}

pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, (StatusCode, Json<serde_json::Value>)> {
    match state.processor.get_task(id).await {
        Some(task) => Ok(Json(task)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Tarefa não encontrada"
            })),
        )),
    }
}

pub async fn list_tasks(
    State(state): State<AppState>,
) -> Json<Vec<Task>> {
    let tasks = state.processor.list_tasks().await;
    Json(tasks)
}

pub async fn cancel_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.processor.cancel_task(id).await {
        Ok(_) => Ok(Json(serde_json::json!({
            "message": "Tarefa cancelada com sucesso",
            "task_id": id
        }))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": e
            })),
        )),
    }
}

pub async fn get_stats(
    State(state): State<AppState>,
) -> Json<TaskStats> {
    let stats = state.processor.get_stats();
    Json(stats)
}

