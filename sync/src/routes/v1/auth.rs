use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OauthQueryParams {
    code: String,
}

pub async fn oauth(query: Query<OauthQueryParams>) -> String {
    query.code.to_string()
}
