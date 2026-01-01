use axum::{
    extract::{
        ws::{Message, WebSocket},
        State,
    },
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use crate::AppState;

pub async fn handle_websocket(
    ws: WebSocket,
    State(state): State<AppState>,
) {
    let (sender, mut receiver) = ws.split();
    let sender = Arc::new(Mutex::new(sender));
    let mut rx = state.processor.subscribe();

    info!("🔌 Nova conexão WebSocket estabelecida");

    let sender_clone = sender.clone();
    // Spawn task para enviar atualizações
    let mut send_task = tokio::spawn(async move {
        while let Ok((_id, task)) = rx.recv().await {
            let message = serde_json::json!({
                "type": "task_update",
                "task": task
            });

            let mut sender = sender_clone.lock().await;
            if let Err(e) = sender.send(Message::Text(
                serde_json::to_string(&message).unwrap(),
            )).await {
                warn!("Erro ao enviar mensagem WebSocket: {}", e);
                break;
            }
        }
    });

    // Spawn task para receber mensagens (ping/pong)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    info!("Conexão WebSocket fechada pelo cliente");
                    break;
                }
                Message::Ping(payload) => {
                    // Responder com pong
                    let mut sender = sender.lock().await;
                    if let Err(e) = sender.send(Message::Pong(payload)).await {
                        error!("Erro ao enviar pong: {}", e);
                        break;
                    }
                }
                _ => {}
            }
        }
    });

    // Aguardar uma das tasks terminar
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
        }
    }
}

