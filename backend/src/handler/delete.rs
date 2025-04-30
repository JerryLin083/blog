use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
      delete from users where user_id = $1
    "#;

    sqlx::query(query_str)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(String::from("Ok"))
}
pub async fn delete_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
      delete from posts where id = $1
    "#;

    sqlx::query(query_str)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(String::from("Ok"))
}
