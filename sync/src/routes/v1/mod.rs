use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use axum_macros::FromRequest;
use core::fmt;
use serde::Serialize;
use std::fmt::Debug;
use tracing::warn;

pub mod codes;
pub mod misc;
pub mod users;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
    #[serde(rename = "errorKind")]
    pub kind: String,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    MissingContentType,
    JsonSyntaxError,
    JsonDataError,
    JsonUnknownError,
    DatabaseError(sqlx::Error),
    MissingAuthentication,
    InvalidAuthentication,
    InvalidJwtSignature,
    JwtUserGone,
    /// Usually caused by giving Iceblink an invalid authentication token.
    /// Still logging a warning regardless.
    OpenIdTokenExchangeFail(reqwest::Error),
    /// This should generally not happen, since we have received an authenticated token from the IdP.
    OpenIdUserinfoFail(reqwest::Error),
    NoIcon,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found."),
			ApiError::MissingContentType => (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported Content-Type. Did you mean to set it to applicaiton/json?"),
			ApiError::JsonSyntaxError => (StatusCode::BAD_REQUEST, "Unable to parse JSON request."),
			ApiError::JsonDataError => (StatusCode::BAD_REQUEST, "Unable to process JSON. Are you missing a field? Tip: Check with the swagger documentation at /swagger!"),
			ApiError::JsonUnknownError => (StatusCode::INTERNAL_SERVER_ERROR, "Unable to parse and process JSON contents. Try again later."),
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
			ApiError::JwtUserGone => (StatusCode::UNAUTHORIZED, "Authenticated user does not exist. Has the account been deleted?"),
			ApiError::OpenIdTokenExchangeFail(err) => {
				warn!("Failed to exchange from IdP: {err}");
				(StatusCode::BAD_REQUEST, "Failed to exchange token with authentication provider. Please make sure to not edit the URL. Please try again.")
			},
			ApiError::OpenIdUserinfoFail(err) => {
				warn!("Failed to get userinfo from IdP: {err}");
				(StatusCode::INTERNAL_SERVER_ERROR, "Failed to aquire userinfo from authentication provider. Try again later.")
			},
			ApiError::NoIcon => (StatusCode::NO_CONTENT, "Unable to find an icon for this code. Double check your website URL.")
        };

        (
            status,
            axum::Json(ApiErrorResponse {
                message: message.to_string(),
                kind: self.kind(),
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
    fn kind(&self) -> String {
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

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct JSON<T>(T);

impl<T> IntoResponse for JSON<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

impl From<JsonRejection> for ApiError {
    fn from(value: JsonRejection) -> Self {
        match value {
            JsonRejection::JsonDataError(_) => ApiError::JsonDataError,
            JsonRejection::JsonSyntaxError(_) => ApiError::JsonSyntaxError,
            JsonRejection::MissingJsonContentType(_) => ApiError::MissingContentType,
            _ => ApiError::JsonUnknownError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_get_kind_missing_authentication() {
        assert_that!(
            ApiError::MissingAuthentication.kind(),
            eq("MissingAuthentication")
        );
    }

    #[gtest]
    fn test_get_kind_sqlx_misc() {
        assert_that!(
            ApiError::DatabaseError(sqlx::Error::ColumnNotFound("joe".to_string())).kind(),
            eq("DatabaseError")
        );
    }

    #[gtest]
    fn test_get_kind_not_found_from_sqlx() {
        assert_that!(
            ApiError::from(sqlx::Error::RowNotFound).kind(),
            eq("NotFound")
        );
    }
}
