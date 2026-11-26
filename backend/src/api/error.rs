use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::domain::error::RepositoryError;
use crate::service::auth::AuthError;
use crate::service::comment::CommentServiceError;
use crate::service::post::ServiceError;
use crate::service::setting::SettingServiceError;
use crate::service::stats::StatsError;

#[derive(Debug)]
pub(super) enum ApiError {
    NotFound,
    Unauthorized(String),
    NotConfigured,
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

impl From<CommentServiceError> for ApiError {
    fn from(error: CommentServiceError) -> Self {
        match error {
            CommentServiceError::PostNotFound => Self::NotFound,
            CommentServiceError::NotFound => Self::NotFound,
            CommentServiceError::Validation(message) => Self::Validation(message),
            CommentServiceError::Repository(RepositoryError::Conflict(message)) => {
                Self::Conflict(message)
            }
            CommentServiceError::Repository(error) => {
                tracing::error!(error = %error, "comment repository request failed");
                Self::Internal
            }
        }
    }
}

impl From<SettingServiceError> for ApiError {
    fn from(error: SettingServiceError) -> Self {
        match error {
            SettingServiceError::Validation(message) => Self::Validation(message),
            SettingServiceError::Repository(RepositoryError::Conflict(message)) => {
                Self::Conflict(message)
            }
            SettingServiceError::Repository(error) => {
                tracing::error!(error = %error, "settings repository request failed");
                Self::Internal
            }
        }
    }
}

impl From<StatsError> for ApiError {
    fn from(error: StatsError) -> Self {
        match error {
            StatsError::Repository(RepositoryError::Conflict(message)) => Self::Conflict(message),
            StatsError::Repository(error) => {
                tracing::error!(error = %error, "stats repository request failed");
                Self::Internal
            }
        }
    }
}

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InvalidCredentials => Self::Unauthorized("用户名或密码错误".to_string()),
            AuthError::AccountLocked => Self::Unauthorized("账户已临时锁定，请稍后再试".to_string()),
            AuthError::NotConfigured => Self::NotConfigured,
            AuthError::Token(message) => {
                tracing::error!(error = %message, "failed to issue admin token");
                Self::Internal
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "post not found".to_string()),
            Self::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            Self::NotConfigured => (
                StatusCode::SERVICE_UNAVAILABLE,
                "admin auth is not configured".to_string(),
            ),
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
