use anyhow::{anyhow, Result};
use futures::stream::Stream;
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::StreamExt;

const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";

fn ollama_base_url() -> String {
    std::env::var("OLLAMA_URL").unwrap_or_else(|_| DEFAULT_OLLAMA_URL.to_string())
}

pub async fn generate_sync(
    model: &str,
    prompt: &str,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
) -> Result<String> {
    let client = Client::new();
    let url = format!("{}/api/generate", ollama_base_url());
    let payload = json!({
        "model": model,
        "prompt": prompt,
        "max_tokens": max_tokens.unwrap_or(512),
        "temperature": temperature.unwrap_or(0.7),
        "top_p": top_p.unwrap_or(1.0)
    });

    let resp = client
        .post(&url)
        .json(&payload)
        .send()
        .await?;

    if resp.status().is_success() {
        let text = resp.text().await?;
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(t) = v.get("response").and_then(|t| t.as_str()) {
                return Ok(t.to_string());
            }
        }
        Ok(text)
    } else {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        Err(anyhow!("Ollama error {}: {}", status, body))
    }
}

pub async fn stream_generate(
    model: &str,
    prompt: &str,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
) -> Result<Pin<Box<dyn Stream<Item = Result<String, anyhow::Error>> + Send>>> {
    let client = Client::new();
    let url = format!("{}/api/generate", ollama_base_url());
    let payload = json!({
        "model": model,
        "prompt": prompt,
        "max_tokens": max_tokens.unwrap_or(512),
        "temperature": temperature.unwrap_or(0.7),
        "top_p": top_p.unwrap_or(1.0),
        "stream": true
    });

    let mut resp = client.post(&url).json(&payload).send().await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!("Ollama stream error {}: {}", status, body));
    }

    let (tx, rx): (
        tokio::sync::mpsc::UnboundedSender<Result<String, anyhow::Error>>,
        UnboundedReceiver<Result<String, anyhow::Error>>,
    ) = unbounded_channel();

    tokio::spawn(async move {
        loop {
            match resp.chunk().await {
                Ok(Some(chunk)) => {
                    let s = String::from_utf8_lossy(&chunk).to_string();
                    for part in s.split('\n').filter(|p| !p.is_empty()) {
                        let sent = if let Ok(v) = serde_json::from_str::<serde_json::Value>(part)
                        {
                            if let Some(t) = v.get("response").and_then(|t| t.as_str()) {
                                Ok(t.to_string())
                            } else if v.get("done").and_then(|d| d.as_bool()) == Some(true) {
                                let final_info = serde_json::to_string(&v).unwrap_or_default();
                                Ok(final_info)
                            } else {
                                Ok(part.to_string())
                            }
                        } else {
                            Ok(part.to_string())
                        };
                        if tx.send(sent).is_err() {
                            tracing::info!("stream receiver dropped");
                            return;
                        }
                    }
                }
                Ok(None) => {
                    tracing::info!("ollama stream completed");
                    return;
                }
                Err(e) => {
                    let _ = tx.send(Err(anyhow!("error reading stream chunk: {:?}", e)));
                    return;
                }
            }
        }
    });

    let stream = UnboundedReceiverStream::new(rx).map(|res| res);
    Ok(Box::pin(stream))
}

pub async fn list_models() -> Result<Vec<String>> {
    let client = Client::new();
    let url = format!("{}/api/tags", ollama_base_url());

    let resp = client.get(&url).send().await?;

    if resp.status().is_success() {
        let body = resp.text().await?;
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&body) {
            let models = v
                .get("models")
                .and_then(|m| m.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|m| m.get("name").and_then(|n| n.as_str()))
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();
            return Ok(models);
        }
    }
    Ok(vec![])
}
