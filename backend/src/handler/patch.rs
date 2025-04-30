use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use super::User;

pub async fn patch_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(user): Json<User>,
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

    sqlx::query(query_str)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.address)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(String::from("Ok"))
}
pub async fn patch_post() {}
