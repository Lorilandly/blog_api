mod controllers;
mod models;

use axum::routing::{get, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or_else(|_| 3000.to_string());
    let addr = format!("0.0.0.0:{}", port);
    let db_url = env::var("DATABASE_URL").expect("missing env: DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    models::init(&pool).await?;

    let app = Router::new()
        .route(
            "/articles",
            get(controllers::articles::read_all_article_id)
                .post(controllers::articles::create_article),
        )
        .route(
            "/articles/:id",
            get(controllers::articles::read_article)
                .put(controllers::articles::update_article)
                .delete(controllers::articles::delete_article),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
