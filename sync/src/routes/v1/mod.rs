use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use tracing::warn;

pub mod codes;
pub mod index;
pub mod user;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    DatabaseError(sqlx::Error),
    MissingAuthentication,
    JwtInvalid,
    JwtUserGone,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found."),
            ApiError::DatabaseError(err) => {
				warn!("Database error occoured: {}", err);
				(StatusCode::INTERNAL_SERVER_ERROR, "Internal database error. Try again later.")
			},
            ApiError::MissingAuthentication => (
                StatusCode::UNAUTHORIZED,
                "Missing authentication. Supply a JWT in the `iceblink_jwt` cookie, or use a bearer in the `Authorization` header.",
            ),
			ApiError::JwtInvalid => (StatusCode::UNAUTHORIZED, "The supplied authentication is invalid."),
			ApiError::JwtUserGone => (StatusCode::UNAUTHORIZED, "Authenticated user does not exist. Has the account been deleted?")
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
            _ => ApiError::DatabaseError(value),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        ApiError::JwtInvalid
    }
}
