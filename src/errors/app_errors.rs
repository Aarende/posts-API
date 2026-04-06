use axum::{
    Json,
    http::StatusCode,
    response::IntoResponse
};
use serde_json::json;
use thiserror::Error;
use super::db_errors::*;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Resource with the same name already exists")]
    Conflict,

    #[error("Invalid input")]
    BadRequest,

    #[error("The user is unauthorized")]
    Unauthorized,

    #[error("Access to the resource is forbidden")]
    Forbidden,

    #[error("Internal error")]
    Internal
}

impl From<GetError> for AppError {
    fn from(value: GetError) -> Self {
        match value {
            GetError::NotFound => AppError::NotFound,
            GetError::Internal => AppError::Internal
        }
    }
}

impl From<PostError> for AppError {
    fn from(value: PostError) -> Self {
        match value {
            PostError::Conflict => AppError::Conflict,
            PostError::Internal => AppError::Internal
        }
    }
}

impl From<PatchError> for AppError {
    fn from(value: PatchError) -> Self {
        match value {
            PatchError::Conflict => AppError::Conflict,
            PatchError::NotFound => AppError::NotFound,
            PatchError::Internal => AppError::Internal
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(_value: diesel::result::Error) -> Self {
        AppError::Internal
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, json!({ "error": "resource not found" })),
            AppError::Conflict => (StatusCode::CONFLICT, json!({ "error": "resource with the same name already exists" })),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, json!({ "error": "invalid input" })),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, json!({ "error": "the user is unauthorized" })),
            AppError::Forbidden => (StatusCode::FORBIDDEN, json!({ "error": "the user have no access to the resource" })),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, json!({ "error": "internal error" })),
        };

        (status, Json(body)).into_response()
    }
}
