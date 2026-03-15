use lambda_http::{Body, Response};
use crate::models::{ErrorDetail, ErrorResponse};

#[derive(Debug)]
pub enum AppError {
    Validation { message: String },
    VoiceMapping { message: String },
    VoicevoxApi { message: String },
    Internal { message: String },
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::Validation { .. } => 400,
            AppError::VoiceMapping { .. } => 400,
            AppError::VoicevoxApi { .. } => 500,
            AppError::Internal { .. } => 500,
        }
    }

    pub fn error_type(&self) -> &str {
        match self {
            AppError::Validation { .. } => "invalid_request_error",
            AppError::VoiceMapping { .. } => "invalid_request_error",
            AppError::VoicevoxApi { .. } => "server_error",
            AppError::Internal { .. } => "server_error",
        }
    }

    pub fn code(&self) -> &str {
        match self {
            AppError::Validation { .. } => "invalid_request",
            AppError::VoiceMapping { .. } => "invalid_voice",
            AppError::VoicevoxApi { .. } => "voicevox_error",
            AppError::Internal { .. } => "internal_error",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            AppError::Validation { message } => message,
            AppError::VoiceMapping { message } => message,
            AppError::VoicevoxApi { message } => message,
            AppError::Internal { message } => message,
        }
    }

    pub fn to_response(&self) -> Response<Body> {
        let error_response = ErrorResponse {
            error: ErrorDetail {
                message: self.message().to_string(),
                error_type: self.error_type().to_string(),
                code: self.code().to_string(),
            },
        };

        let body = serde_json::to_string(&error_response).unwrap_or_else(|_| {
            r#"{"error":{"message":"Internal error","type":"server_error","code":"internal_error"}}"#
                .to_string()
        });

        Response::builder()
            .status(self.status_code())
            .header("content-type", "application/json")
            .body(Body::Text(body))
            .unwrap()
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type(), self.message())
    }
}

impl std::error::Error for AppError {}
