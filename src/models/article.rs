use chrono::DateTime;
use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, PgPool, Result, Row};
use uuid::Uuid;

static ARTICLE_ALL_ID: &str = "
    select id from articles
";

#[derive(Debug, Serialize, FromRow)]
pub struct Article {
    id: Uuid,
    title: String,
    auther_id: Uuid,
    body: String,
    created_at: DateTime<chrono::Utc>,
    updated_at: DateTime<chrono::Utc>,
}

impl Article {
    pub fn new(title: String, body: String, auther_id: Uuid) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            auther_id,
            body,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn persist(&self, pool: &PgPool) -> Result<&Self> {
        sqlx::query!(
            "\
            INSERT INTO articles (id, title, auther_id, body, created_at, updated_at) \
            VALUES ($1, $2, $3, $4, $5, $6) \
            ",
            &self.id,
            &self.title,
            &self.auther_id,
            &self.body,
            &self.created_at,
            &self.updated_at
        )
        .execute(pool)
        .await?;

        Ok(self)
    }

    pub async fn read(pool: &PgPool, id: Uuid) -> Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM articles WHERE id=$1", id)
            .fetch_optional(pool)
            .await
    }

    pub async fn read_all_id(pool: &PgPool) -> Result<Vec<Uuid>> {
        let ids = sqlx::query(ARTICLE_ALL_ID)
            .try_map(|row: PgRow| row.try_get("id"))
            .fetch_all(pool)
            .await?;
        Ok(ids)
    }

    pub async fn update(pool: &PgPool, id: Uuid, title: String, body: String) -> Result<u64> {
        let now = chrono::Utc::now();
        Ok(sqlx::query!(
            "\
            UPDATE articles \
            SET title = $1, body = $2, updated_at = $3 \
            WHERE id = $4 \
            ",
            title,
            body,
            now,
            id,
        )
        .execute(pool)
        .await?
        .rows_affected())
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<u64> {
        Ok(sqlx::query!(
            "\
            DELETE FROM articles \
            WHERE id = $1 \
            ",
            id,
        )
        .execute(pool)
        .await?
        .rows_affected())
    }
}
