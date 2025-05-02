use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use super::{PostRequest, UserRequest};

pub async fn patch_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(user): Json<UserRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
        update users set
        first_name = $1,
        last_name = $2,
        email = $3,
        phone = $4,
        address = $5
        where user_id = $6
      "#;

    let result = sqlx::query(query_str)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.address)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .rows_affected();

    Ok((StatusCode::OK, result.to_string()))
}

pub async fn publish_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
      update posts set
      update_at = NOW(),
      published_at = NOW()
      where id = $1
    "#;

    let result = sqlx::query(query_str)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .rows_affected();

    Ok((StatusCode::OK, result.to_string()))
}

//TODO: update post by id
pub async fn edit_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(post): Json<PostRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
        update posts set 
        title = $1,
        content = $2,
        update_at = NOW()
        where id = $3
    "#;

    let result = sqlx::query(query_str)
        .bind(&post.title)
        .bind(&post.content)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .rows_affected();

    Ok((StatusCode::OK, result.to_string()))
}
