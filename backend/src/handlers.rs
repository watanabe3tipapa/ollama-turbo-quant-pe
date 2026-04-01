use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Json,
    response::IntoResponse,
};
use futures::StreamExt;
use serde_json::json;

use crate::ollama_client;
use crate::types::{GenerateRequest, GenerateResponse, ModelsResponse};

pub async fn generate_sync(Json(req): Json<GenerateRequest>) -> impl IntoResponse {
    let model = req
        .model
        .clone()
        .unwrap_or_else(|| std::env::var("DEFAULT_MODEL").unwrap_or_else(|_| "llama3.2".into()));
    match ollama_client::generate_sync(
        &model,
        &req.prompt,
        req.max_tokens,
        req.temperature,
        req.top_p,
    )
    .await
    {
        Ok(text) => (axum::http::StatusCode::OK, Json(GenerateResponse { text })).into_response(),
        Err(e) => {
            tracing::error!("generate_sync error: {:?}", e);
            let body = json!({ "error": format!("{}", e) });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        }
    }
}

pub async fn list_models_handler() -> impl IntoResponse {
    match ollama_client::list_models().await {
        Ok(models) => (axum::http::StatusCode::OK, Json(ModelsResponse { models })).into_response(),
        Err(e) => {
            tracing::error!("list_models error: {:?}", e);
            let body = json!({ "error": format!("{}", e) });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
        }
    }
}

pub async fn ws_generate(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(txt) => {
                match serde_json::from_str::<GenerateRequest>(&txt) {
                    Ok(init) => {
                        let model = init.model.unwrap_or_else(|| {
                            std::env::var("DEFAULT_MODEL").unwrap_or_else(|_| "llama3.2".into())
                        });
                        match ollama_client::stream_generate(
                            &model,
                            &init.prompt,
                            init.max_tokens,
                            init.temperature,
                            init.top_p,
                        )
                        .await
                        {
                            Ok(mut stream) => {
                                while let Some(item) = stream.next().await {
                                    match item {
                                        Ok(chunk_text) => {
                                            if socket
                                                .send(Message::Text(chunk_text))
                                                .await
                                                .is_err()
                                            {
                                                tracing::info!("client disconnected");
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            let _ = socket
                                                .send(Message::Text(format!("__ERROR__:{:?}", e)))
                                                .await;
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = socket
                                    .send(Message::Text(format!(
                                        "__ERROR__: failed to start ollama stream: {:?}",
                                        e
                                    )))
                                    .await;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = socket
                            .send(Message::Text(format!("__ERROR__: invalid init json: {:?}", e)))
                            .await;
                    }
                }
            }
            Message::Close(_) | Message::Binary(_) | Message::Ping(_) | Message::Pong(_) => {}
        }
    } else {
        tracing::info!("no init message; closing socket");
    }
}
