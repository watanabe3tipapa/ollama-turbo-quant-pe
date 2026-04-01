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

#[derive(Debug, Deserialize, Serialize)]
pub struct EnhanceRequest {
    pub text: String,
    #[serde(default)]
    pub params: Option<EnhanceParams>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct EnhanceParams {
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

fn default_temperature() -> f32 {
    0.7
}

fn default_top_p() -> f32 {
    1.0
}

#[derive(Debug, Serialize)]
pub struct EnhanceResponse {
    pub original: String,
    pub enhanced: String,
    pub tokens: usize,
    #[serde(rename = "compressionRatio")]
    pub compression_ratio: f32,
}
