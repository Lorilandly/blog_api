use crate::models::article::Article;
use axum::{extract, http::StatusCode, response};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct CreateArticle {
    title: String,
    body: String,
    author_id: uuid::Uuid,
}

pub async fn post_articles(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateArticle>,
) -> Result<impl response::IntoResponse, StatusCode> {
    let article = Article::new(payload.title, payload.body, payload.author_id);
    println!("post endpoint");

    match article.persist(&pool).await {
        Ok(_) => {
            println!("{:?}", axum::Json(&article));
            Ok(axum::Json(article))
        }
        Err(err) => {
            println!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read_articles(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<uuid::Uuid>>, StatusCode> {
    println!("get endpoint");
    match Article::read_all_id(&pool).await {
        Ok(ids) => Ok(axum::Json(ids)),
        Err(err) => {
            println!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
