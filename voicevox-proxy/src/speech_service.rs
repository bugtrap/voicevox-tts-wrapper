use crate::error::AppError;
use crate::models::SpeechRequest;
use crate::voice_mapping::VoiceMapping;
use crate::voicevox_client::VoicevoxClient;

pub fn validate_request(req: &SpeechRequest) -> Result<(), AppError> {
    if req.model.is_empty() {
        return Err(AppError::Validation {
            message: "Missing required field: model".to_string(),
        });
    }
    if req.input.is_empty() {
        return Err(AppError::Validation {
            message: "Input text must not be empty".to_string(),
        });
    }
    if req.voice.is_empty() {
        return Err(AppError::Validation {
            message: "Missing required field: voice".to_string(),
        });
    }
    if req.response_format != "wav" {
        return Err(AppError::Validation {
            message: format!(
                "Unsupported response format: '{}'. Only 'wav' is supported",
                req.response_format
            ),
        });
    }
    if !(0.25..=4.0).contains(&req.speed) {
        return Err(AppError::Validation {
            message: "Speed must be between 0.25 and 4.0".to_string(),
        });
    }
    Ok(())
}

pub async fn synthesize(
    req: &SpeechRequest,
    voicevox: &VoicevoxClient,
    mapping: &VoiceMapping,
) -> Result<Vec<u8>, AppError> {
    validate_request(req)?;

    let speaker_id = mapping.resolve(&req.model, &req.voice)?;

    let mut query = voicevox.audio_query(&req.input, speaker_id).await?;

    // Apply speed parameter
    if let Some(obj) = query.as_object_mut() {
        obj.insert("speedScale".to_string(), serde_json::json!(req.speed));

        if let Some(pitch) = req.voicevox_pitch_scale {
            obj.insert("pitchScale".to_string(), serde_json::json!(pitch));
        }
        if let Some(intonation) = req.voicevox_intonation_scale {
            obj.insert("intonationScale".to_string(), serde_json::json!(intonation));
        }
        if let Some(volume) = req.voicevox_volume_scale {
            obj.insert("volumeScale".to_string(), serde_json::json!(volume));
        }
    }

    voicevox.synthesis(&query, speaker_id).await
}
