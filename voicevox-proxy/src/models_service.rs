use crate::models::{ModelObject, ModelsResponse};
use crate::voice_mapping::VoiceMapping;

pub fn list_models(mapping: &VoiceMapping) -> ModelsResponse {
    let data = mapping
        .models()
        .into_iter()
        .map(|id| ModelObject {
            id,
            object: "model".to_string(),
            created: 0,
            owned_by: "voicevox".to_string(),
        })
        .collect();

    ModelsResponse {
        object: "list".to_string(),
        data,
    }
}
