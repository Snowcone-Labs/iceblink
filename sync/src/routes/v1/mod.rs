use core::fmt;
use std::fmt::Debug;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use tracing::warn;

pub mod codes;
pub mod index;
pub mod user;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    DatabaseError(sqlx::Error),
    MissingAuthentication,
    InvalidAuthentication,
    InvalidJwtSignature,
    JwtUserGone,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found."),
            ApiError::DatabaseError(err) => {
				warn!("Database error occoured: {}", err);
				(StatusCode::INTERNAL_SERVER_ERROR, "Internal database error. Try again later.")
			},
            ApiError::MissingAuthentication => (
                StatusCode::UNAUTHORIZED,
                "Missing authentication. Supply a JWT in the `iceblink_jwt` cookie, or use a bearer in the `Authorization` header.",
            ),
			ApiError::InvalidAuthentication => (StatusCode::UNAUTHORIZED, "The supplied authentication is invalid."),
			ApiError::InvalidJwtSignature => (StatusCode::UNAUTHORIZED, "The supplied authentication has an invalid signature. Try logging in again."),
			ApiError::JwtUserGone => (StatusCode::UNAUTHORIZED, "Authenticated user does not exist. Has the account been deleted?")
        };

        (
            status,
            Json(ApiErrorResponse {
                message: message.to_string(),
                typ: self.typ(),
            }),
        )
            .into_response()
    }
}

// Get enum name to present in `type` field
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

// Stupid way to get enum name without contents
impl ApiError {
    fn typ(&self) -> String {
        self.to_string()
            .split_once("(")
            .unwrap_or((self.to_string().as_str(), ""))
            .0
            .to_string()
    }
}

// Converting errors from other crates
impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::DatabaseError(value),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        match value.into_kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature
            | jsonwebtoken::errors::ErrorKind::InvalidSignature => ApiError::InvalidJwtSignature,
            _ => ApiError::InvalidAuthentication,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_typ_missing_authentication() {
        assert_eq!(
            ApiError::MissingAuthentication.typ(),
            "MissingAuthentication"
        );
    }

    #[test]
    fn test_get_type_sqlx() {
        assert_eq!(
            ApiError::DatabaseError(sqlx::Error::ColumnNotFound("joe".to_string())).typ(),
            "DatabaseError"
        );
    }
}
