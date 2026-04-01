use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateRequest {
    pub prompt: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(rename = "max_tokens")]
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(rename = "top_p")]
    #[serde(default)]
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub models: Vec<String>,
}
