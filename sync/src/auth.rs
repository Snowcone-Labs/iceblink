use crate::{
    models::{self, user::User},
    routes::v1::ApiError,
    AppState,
};
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use reqwest::header::USER_AGENT;
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

pub async fn create_jwt(user: &User, secret: String) -> (String, Cookie) {
    let now = chrono::Utc::now();

    let claims = TokenClaims {
        iat: now.timestamp() as usize,
        exp: (now + chrono::Duration::days(90)).timestamp() as usize,
        sub: user.id.clone(),
        username: user.username.clone(),
        display_name: user.display_name.clone(),
        avatar_url: user.avatar_url.clone(),
    };

    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("iceblink_jwt", jwt.clone()))
        .same_site(SameSite::Strict)
        .secure(true)
        .http_only(true)
        .build();

    (jwt, cookie)
}

pub async fn jwt_middleware(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let token = cookie_jar
        .get("iceblink_jwt")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer "))
                .map(str::to_string)
        })
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().to_string());

    let token = token.ok_or(ApiError::MissingAuthentication)?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.settings.jwt_secret.as_ref()),
        &Validation::default(),
    )?
    .claims;

    let user = models::user::User::get_by_id(&data.db, claims.sub).await?;
    let user = user.ok_or(ApiError::JwtUserGone)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

#[derive(Deserialize, Clone)]
pub struct OpenIdDiscovery {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OpenIdUserInfo {
    #[serde(rename = "sub")]
    pub id: String,
    #[serde(rename = "name")]
    pub display_name: Option<String>,
    #[serde(rename = "preferred_username")]
    pub username: String,
    #[serde(rename = "picture")]
    pub avatar: String,
}

#[derive(Deserialize, Debug)]
struct TokenExchangeResponse {
    access_token: String,
}

#[derive(Clone)]
pub struct OpenId {
    pub authorization: String,
    pub token: String,
    pub userinfo: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Debug)]
struct TokenExchangeRequest {
    client_id: String,
    client_secret: String,
    code: String,
}

#[bon::bon]
impl OpenId {
    #[builder]
    pub async fn discover(
        client_id: String,
        client_secret: String,
        server: String,
    ) -> Result<Self, reqwest::Error> {
        let config = reqwest::get(format!("{server}/.well-known/openid-configuration"))
            .await?
            .json::<OpenIdDiscovery>()
            .await?;

        Ok(OpenId {
            client_id,
            client_secret,
            authorization: config.authorization_endpoint,
            token: config.token_endpoint,
            userinfo: config.userinfo_endpoint,
        })
    }

    pub async fn exchange(self, code: String) -> Result<String, reqwest::Error> {
        let request = reqwest::Client::new()
            .post(self.token)
            .header(USER_AGENT, "Iceblink")
            .json(&TokenExchangeRequest {
                client_id: self.client_id,
                client_secret: self.client_secret,
                code,
            });

        let response = request
            .send()
            .await?
            .json::<TokenExchangeResponse>()
            .await?;

        Ok(response.access_token)
    }

    pub async fn userinfo(self, token: String) -> Result<OpenIdUserInfo, reqwest::Error> {
        let request = reqwest::Client::new()
            .get(self.userinfo)
            .header(USER_AGENT, "Iceblink")
            .bearer_auth(token);

        request.send().await?.json::<OpenIdUserInfo>().await
    }
}
