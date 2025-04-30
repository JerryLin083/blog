use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::{PgPool, Row};

use super::{Post, User};

pub async fn create_user(
    State(pool): State<PgPool>,
    user: Json<User>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
        insert into users(first_name, last_name, email, phone, address)
        values($1, $2, $3, $4, $5) returning user_id, first_name, last_name
        "#;

    let row = sqlx::query(query_str)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.address)
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let created = CreatedUser {
        user_id: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
    };

    Ok((StatusCode::CREATED, Json(created)))
}

#[derive(Serialize)]
struct CreatedUser {
    user_id: i32,
    first_name: String,
    last_name: String,
}

pub async fn create_post(
    State(pool): State<PgPool>,
    post: Json<Post>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
        insert into posts(title, content, user_id)
        values($1, $2, $3)
        returning id, title
    "#;

    let row = sqlx::query(query_str)
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.user_id)
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let create_post = CreatePost {
        id: row.get(0),
        title: row.get(1),
    };

    Ok(Json(create_post))
}

#[derive(Serialize)]
struct CreatePost {
    id: i32,
    title: String,
}
