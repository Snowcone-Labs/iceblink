use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub mod auth;
pub mod codes;
pub mod index;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    DatabaseError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resources not found"),
            ApiError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };

        (
            status,
            Json(ApiErrorResponse {
                message: message.to_string(),
            }),
        )
            .into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::DatabaseError,
        }
    }
}
