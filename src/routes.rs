use crate::controllers::articles;
use axum::routing::{get, Router};
use sqlx::PgPool;

pub fn routes(state: &PgPool) -> Router {
    Router::new()
        .route(
            "/articles",
            get(articles::read_all_article_id).post(articles::create_article),
        )
        .route(
            "/articles/:id",
            get(articles::read_article)
                .put(articles::update_article)
                .delete(articles::delete_article),
        )
        .with_state(state.clone())
}
