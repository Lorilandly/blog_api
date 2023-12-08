mod controllers;
mod models;
mod routes;

use axum::routing::Router;
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

    sqlx::migrate!().run(&pool).await?;

    let app = Router::new().nest("/api", routes::routes(&pool));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
