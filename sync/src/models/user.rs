use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub upstream_userid: String,
}

impl User {
    pub async fn get_by_id(
        pool: &SqlitePool,
        id: String,
    ) -> Result<Option<User>, sqlx::error::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(pool)
            .await
    }

    pub async fn get_by_upstream_id(
        pool: &SqlitePool,
        id: String,
    ) -> Result<Option<User>, sqlx::error::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE upstream_userid = ?", id)
            .fetch_optional(pool)
            .await
    }

    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
			"INSERT INTO users (id, username, display_name, avatar_url, upstream_userid) VALUES ($1, $2, $3, $4, $5)",
			self.id, self.username, self.display_name, self.avatar_url, self.upstream_userid).execute(pool).await?;

        Ok(())
    }
}
