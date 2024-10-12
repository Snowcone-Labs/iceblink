use crate::{
    models::{self, user::User},
    server::AppState,
};
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
}

pub async fn create_jwt(user: &User, secret: String) -> String {
    let now = chrono::Utc::now();

    let claims = TokenClaims {
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::days(90)).timestamp() as usize,
        sub: user.id.clone(),
        username: user.username.clone(),
        display_name: user.display_name.clone(),
        avatar_url: user.avatar_url.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

#[derive(Debug, Serialize)]
pub struct JwtMiddlewareError {
    pub message: String,
}

pub async fn jwt_middleware(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<JwtMiddlewareError>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
                .map(|auth_value_inner| auth_value_inner.to_string())
        });

    let token = token.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(JwtMiddlewareError {
                message: "You are not logged in, please provide token".to_string(),
            }),
        )
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.settings.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = JwtMiddlewareError {
            message: "Invalid token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    let user = models::user::User::get_by_id(&data.db, claims.sub)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(JwtMiddlewareError {
                    message: format!("Error fetching user from database: {}", e),
                }),
            )
        })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(JwtMiddlewareError {
                message: "The user belonging to this token no longer exists".to_string(),
            }),
        )
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

#[derive(Deserialize, Clone)]
pub struct OpenId {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
}

impl OpenId {
    pub async fn get(base: String) -> Result<OpenId, Box<dyn std::error::Error>> {
        let response = reqwest::get(format!("{base}/.well-known/openid-configuration"))
            .await?
            .json::<OpenId>()
            .await?;

        Ok(response)
    }
}
