use sqlx::{Error, PgPool};

pub mod article;

pub async fn init(pool: &PgPool) -> Result<(), Error> {
    article::Article::init(pool).await
}
