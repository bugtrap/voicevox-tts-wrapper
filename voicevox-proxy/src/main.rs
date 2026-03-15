mod error;
mod models;
mod models_service;
mod speech_service;
mod voice_mapping;
mod voicevox_client;

use lambda_http::{
    http::Method, run, service_fn, Body, Request, RequestPayloadExt, Response,
};
use std::sync::Arc;
use tracing::info;

use error::AppError;
use models::SpeechRequest;
use voice_mapping::VoiceMapping;
use voicevox_client::VoicevoxClient;

struct AppState {
    voicevox: VoicevoxClient,
    mapping: VoiceMapping,
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .without_time()
        .init();

    let voicevox_url =
        std::env::var("VOICEVOX_API_URL").expect("VOICEVOX_API_URL must be set");

    let state = Arc::new(AppState {
        voicevox: VoicevoxClient::new(&voicevox_url),
        mapping: VoiceMapping::load().expect("Failed to load voice mapping"),
    });

    info!("Lambda initialized");

    run(service_fn(move |event: Request| {
        let state = Arc::clone(&state);
        async move { Ok::<_, lambda_runtime::Error>(handler(event, &state).await) }
    }))
    .await?;

    Ok(())
}

async fn handler(event: Request, state: &AppState) -> Response<Body> {
    let method = event.method().clone();
    let path = event.uri().path().to_string();

    info!(method = %method, path = %path, "Handling request");

    match (method, path.as_str()) {
        (Method::POST, "/v1/audio/speech") => handle_speech(event, state).await,
        (Method::GET, "/v1/models") => handle_models(state),
        _ => AppError::Validation {
            message: "Unknown endpoint".to_string(),
        }
        .to_response(),
    }
}

async fn handle_speech(event: Request, state: &AppState) -> Response<Body> {
    let req: SpeechRequest = match event.payload::<SpeechRequest>() {
        Ok(Some(r)) => r,
        Ok(None) => {
            return AppError::Validation {
                message: "Request body is required".to_string(),
            }
            .to_response();
        }
        Err(e) => {
            return AppError::Validation {
                message: format!("Invalid request body: {}", e),
            }
            .to_response();
        }
    };

    match speech_service::synthesize(&req, &state.voicevox, &state.mapping).await {
        Ok(wav_bytes) => Response::builder()
            .status(200)
            .header("content-type", "audio/wav")
            .body(Body::Binary(wav_bytes))
            .unwrap(),
        Err(e) => {
            tracing::error!(error = %e, "Speech synthesis failed");
            e.to_response()
        }
    }
}

fn handle_models(state: &AppState) -> Response<Body> {
    let response = models_service::list_models(&state.mapping);
    let body = serde_json::to_string(&response).unwrap();
    Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Text(body))
        .unwrap()
}
