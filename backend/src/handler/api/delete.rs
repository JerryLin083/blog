use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use super::{ApiErrorResponse, ApiResponse};

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    let query_str = r#"
      delete from users where user_id = $1
    "#;

    let rows_affected = sqlx::query(query_str)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::from_internal_error(err)),
            )
        })?
        .rows_affected();

    Ok((
        StatusCode::NO_CONTENT,
        Json(ApiResponse::from_ok(rows_affected.to_string())),
    ))
}
pub async fn delete_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    let query_str = r#"
      delete from posts where id = $1
    "#;

    let rows_affected = sqlx::query(query_str)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::from_internal_error(err)),
            )
        })?
        .rows_affected();

    Ok((
        StatusCode::NO_CONTENT,
        Json(ApiResponse::from_ok(rows_affected.to_string())),
    ))
}
