mod api;
mod models;
mod processor;
mod websocket;

use anyhow::Result;
use axum::{
    extract::ws::WebSocketUpgrade,
    extract::State,
    response::Response,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::handlers;
use crate::processor::TaskProcessor;

#[derive(Clone)]
struct AppState {
    processor: Arc<TaskProcessor>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "task_processor=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Iniciando Task Processor Server...");

    // Criar processador de tarefas
    let processor = Arc::new(TaskProcessor::new());
    let state = AppState { processor };

    // Iniciar processamento em background
    let processor_clone = state.processor.clone();
    tokio::spawn(async move {
        processor_clone.start_processing().await;
    });

    // Criar router
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(handlers::health_check))
        .route("/api/tasks", post(handlers::create_task))
        .route("/api/tasks", get(handlers::list_tasks))
        .route("/api/tasks/:id", get(handlers::get_task))
        .route("/api/tasks/:id/cancel", post(handlers::cancel_task))
        .route("/api/stats", get(handlers::get_stats))
        .route("/ws", get(handle_websocket_upgrade))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("📡 Servidor rodando em http://0.0.0.0:3000");
    info!("📊 WebSocket disponível em ws://localhost:3000/ws");
    info!("📚 API REST disponível em http://localhost:3000/api");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    r#"
    🦀 Task Processor API
    
    Endpoints disponíveis:
    - GET  /api/health          - Status do servidor
    - POST /api/tasks           - Criar nova tarefa
    - GET  /api/tasks           - Listar todas as tarefas
    - GET  /api/tasks/:id       - Obter tarefa específica
    - POST /api/tasks/:id/cancel - Cancelar tarefa
    - GET  /api/stats           - Estatísticas do sistema
    - WS   /ws                  - WebSocket para atualizações em tempo real
    
    Exemplo de criação de tarefa:
    POST /api/tasks
    {
        "name": "Processar dados",
        "duration_ms": 5000,
        "priority": "high"
    }
    "#
}

async fn handle_websocket_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket::handle_websocket(socket, State(state)))
}

