use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use sqlx::PgPool;

use crate::session::SessionManager;

use super::{ApiErrorResponse, ApiResponse, PostRequest, UserRequest};

pub async fn patch_user(
    State(pool): State<PgPool>,
    Extension(session_manager): Extension<Arc<SessionManager>>,
    jar: CookieJar,
    Json(user): Json<UserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();
        match session_manager.check_session_id(session_id).await {
            Some(session) => {
                let query_str = r#"
                    update users set
                    first_name = $1,
                    last_name = $2,
                    email = $3,
                    phone = $4,
                    address = $5
                    where user_id = $6
                "#;

                let rows_affected = sqlx::query(query_str)
                    .bind(&user.first_name)
                    .bind(&user.last_name)
                    .bind(&user.email)
                    .bind(&user.phone)
                    .bind(&user.address)
                    .bind(session.user_id)
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
                    StatusCode::OK,
                    Json(ApiResponse::from_ok(rows_affected.to_string())),
                ))
            }
            None => Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse::from_unauthorized("session was expired")),
            )),
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorResponse::from_unauthorized(
                "Authentication required",
            )),
        ))
    }
}

pub async fn publish_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Extension(session_manager): Extension<Arc<SessionManager>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match session_manager.check_session_id(session_id).await {
            Some(session) => {
                let query_str = r#"
                update posts set
                update_at = NOW(),
                published_at = NOW()
                where id = $1 and user_id = $2
                "#;

                let rows_affected = sqlx::query(query_str)
                    .bind(id)
                    .bind(session.user_id)
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
                    StatusCode::OK,
                    Json(ApiResponse::from_ok(rows_affected.to_string())),
                ))
            }
            None => Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse::from_unauthorized("session was expired")),
            )),
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorResponse::from_unauthorized(
                "Authentication required",
            )),
        ))
    }
}

pub async fn edit_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Extension(session_manager): Extension<Arc<SessionManager>>,
    jar: CookieJar,
    Json(post): Json<PostRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match session_manager.check_session_id(session_id).await {
            Some(session) => {
                let query_str = r#"
                    update posts set 
                    title = $1,
                    content = $2,
                    update_at = NOW()
                    where id = $3 and user_id = $4
                "#;

                let rows_affected = sqlx::query(query_str)
                    .bind(&post.title)
                    .bind(&post.content)
                    .bind(id)
                    .bind(session.user_id)
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
                    StatusCode::OK,
                    Json(ApiResponse::from_ok(rows_affected.to_string())),
                ))
            }
            None => Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse::from_unauthorized("session was expired")),
            )),
        }
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorResponse::from_unauthorized(
                "Authentication required",
            )),
        ))
    }
}
