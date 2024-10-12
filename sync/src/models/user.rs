use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserId(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub upstream_userid: String,
}
