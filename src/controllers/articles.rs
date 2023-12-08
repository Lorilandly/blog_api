use crate::models::article::Article;
use axum::{extract, http::StatusCode, response};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateArticle {
    title: String,
    body: String,
    author_id: uuid::Uuid,
}

pub async fn create_article(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateArticle>,
) -> Result<impl response::IntoResponse, StatusCode> {
    let article = Article::new(payload.title, payload.body, payload.author_id);

    match article.persist(&pool).await {
        Ok(_) => Ok(axum::Json(article)),
        Err(err) => {
            println!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read_article(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<Uuid>,
) -> impl response::IntoResponse {
    match Article::read(&pool, id).await {
        Ok(res) => match res {
            Some(article) => Ok(axum::Json(article)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(err) => {
            println!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn read_all_article_id(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<uuid::Uuid>>, StatusCode> {
    match Article::read_all_id(&pool).await {
        Ok(ids) => Ok(axum::Json(ids)),
        Err(err) => {
            println!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_article(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<Uuid>,
    axum::Json(payload): axum::Json<CreateArticle>,
) -> StatusCode {
    match Article::update(&pool, id, payload.title, payload.body).await {
        Ok(n) => match n {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        },
        Err(err) => {
            println!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_article(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<Uuid>,
) -> StatusCode {
    match Article::delete(&pool, id).await {
        Ok(n) => match n {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        },
        Err(err) => {
            println!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
