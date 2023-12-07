use chrono::DateTime;
use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use uuid::Uuid;

static ARTICLE_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS articles (
    id UUID PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    auther_id UUID NOT NULL,
    body TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE(title)
    )
";

static ARTICLE_CREATE: &str = "
    INSERT INTO articles (id, title, auther_id, body, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6)
";

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

    pub async fn init(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(ARTICLE_TABLE).execute(pool).await?;
        Ok(())
    }

    pub async fn persist(&self, pool: &PgPool) -> Result<&Self, sqlx::Error> {
        sqlx::query(ARTICLE_CREATE)
            .bind(&self.id)
            .bind(&self.title)
            .bind(&self.auther_id)
            .bind(&self.body)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(pool)
            .await?;

        Ok(self)
    }

    pub async fn read_all_id(pool: &PgPool) -> Result<Vec<Uuid>, sqlx::Error> {
        let ids = sqlx::query(ARTICLE_ALL_ID)
            .try_map(|row: PgRow| row.try_get("id"))
            .fetch_all(pool)
            .await?;
        Ok(ids)
    }
}