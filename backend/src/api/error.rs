use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::domain::error::RepositoryError;
use crate::service::post::ServiceError;

#[derive(Debug)]
pub(super) enum ApiError {
    NotFound,
    Validation(String),
    Conflict(String),
    Internal,
}

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::NotFound => Self::NotFound,
            ServiceError::Validation(message) => Self::Validation(message),
            ServiceError::Conflict(message) => Self::Conflict(message),
            ServiceError::Repository(RepositoryError::Conflict(message)) => Self::Conflict(message),
            ServiceError::Repository(error) => {
                tracing::error!(error = %error, "repository request failed");
                Self::Internal
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "post not found".to_string()),
            Self::Validation(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            Self::Conflict(message) => (StatusCode::CONFLICT, message),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
