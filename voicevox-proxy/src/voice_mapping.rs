use std::collections::HashMap;
use crate::error::AppError;

const VOICE_MAPPING_JSON: &str = include_str!("../voice-mapping.json");

pub struct VoiceMapping {
    mapping: HashMap<String, HashMap<String, u32>>,
}

impl VoiceMapping {
    pub fn load() -> Result<Self, AppError> {
        let mapping: HashMap<String, HashMap<String, u32>> =
            serde_json::from_str(VOICE_MAPPING_JSON).map_err(|e| AppError::Internal {
                message: format!("Failed to parse voice-mapping.json: {}", e),
            })?;
        Ok(Self { mapping })
    }

    pub fn resolve(&self, model: &str, voice: &str) -> Result<u32, AppError> {
        let voices = self.mapping.get(model).ok_or_else(|| AppError::VoiceMapping {
            message: format!("Model '{}' not found", model),
        })?;
        voices.get(voice).copied().ok_or_else(|| AppError::VoiceMapping {
            message: format!("Voice '{}' not found for model '{}'", voice, model),
        })
    }

    pub fn models(&self) -> Vec<String> {
        self.mapping.keys().cloned().collect()
    }
}
