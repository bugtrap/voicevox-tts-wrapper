use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SpeechRequest {
    pub model: String,
    pub input: String,
    pub voice: String,
    #[serde(default = "default_response_format")]
    pub response_format: String,
    #[serde(default = "default_speed")]
    pub speed: f64,
    pub voicevox_pitch_scale: Option<f64>,
    pub voicevox_intonation_scale: Option<f64>,
    pub voicevox_volume_scale: Option<f64>,
}

fn default_response_format() -> String {
    "wav".to_string()
}

fn default_speed() -> f64 {
    1.0
}

#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub object: String,
    pub data: Vec<ModelObject>,
}

#[derive(Debug, Serialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: String,
}
