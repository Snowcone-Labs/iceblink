use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool};

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct Code {
    pub id: String,
    pub owner_id: String,
    pub content: String,
    pub display_name: String,
    pub icon_url: Option<String>,
    pub website_url: Option<String>,
}

impl Code {
    pub async fn get(
        pool: &SqlitePool,
        id: String,
        owner_id: String,
    ) -> Result<Code, sqlx::error::Error> {
        sqlx::query_as!(
            Code,
            "SELECT * FROM codes WHERE id = ? AND owner_id = ?",
            id,
            owner_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_many(
        pool: &SqlitePool,
        owner_id: String,
    ) -> Result<Vec<Code>, sqlx::error::Error> {
        sqlx::query_as!(Code, "SELECT * FROM codes WHERE owner_id = ?", owner_id)
            .fetch_all(pool)
            .await
    }

    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), sqlx::error::Error> {
        sqlx::query!(
			"INSERT INTO codes (id, owner_id, content, display_name, icon_url, website_url) VALUES ($1, $2, $3, $4, $5, $6)",
			self.id, self.owner_id, self.content, self.display_name, self.icon_url, self.website_url).execute(pool).await?;

        Ok(())
    }

    pub async fn delete(&self, pool: &SqlitePool) -> Result<(), sqlx::error::Error> {
        sqlx::query!("DELETE FROM codes WHERE id = $1", self.id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
