use crate::error::AppError;

pub struct VoicevoxClient {
    base_url: String,
    client: reqwest::Client,
}

impl VoicevoxClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn audio_query(
        &self,
        text: &str,
        speaker_id: u32,
    ) -> Result<serde_json::Value, AppError> {
        let url = format!(
            "{}/audio_query?text={}&speaker={}",
            self.base_url,
            urlencoded(text),
            speaker_id
        );

        let resp = self
            .client
            .post(&url)
            .send()
            .await
            .map_err(|e| AppError::VoicevoxApi {
                message: format!("VOICEVOX audio_query request failed: {}", e),
            })?;

        if !resp.status().is_success() {
            return Err(AppError::VoicevoxApi {
                message: format!("VOICEVOX audio_query failed: {}", resp.status()),
            });
        }

        resp.json::<serde_json::Value>()
            .await
            .map_err(|e| AppError::VoicevoxApi {
                message: format!("Failed to parse audio_query response: {}", e),
            })
    }

    pub async fn synthesis(
        &self,
        query: &serde_json::Value,
        speaker_id: u32,
    ) -> Result<Vec<u8>, AppError> {
        let url = format!("{}/synthesis?speaker={}", self.base_url, speaker_id);

        let resp = self
            .client
            .post(&url)
            .header("content-type", "application/json")
            .json(query)
            .send()
            .await
            .map_err(|e| AppError::VoicevoxApi {
                message: format!("VOICEVOX synthesis request failed: {}", e),
            })?;

        if !resp.status().is_success() {
            return Err(AppError::VoicevoxApi {
                message: format!("VOICEVOX synthesis failed: {}", resp.status()),
            });
        }

        resp.bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| AppError::VoicevoxApi {
                message: format!("Failed to read synthesis response: {}", e),
            })
    }
}

fn urlencoded(s: &str) -> String {
    let mut result = String::new();
    for byte in s.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(*byte as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}
