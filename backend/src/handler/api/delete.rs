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

use super::{ApiErrorResponse, ApiResponse};

pub async fn delete_user(
    State(pool): State<PgPool>,
    Extension(session_manager): Extension<Arc<SessionManager>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match session_manager.check_session_id(session_id).await {
            Some(session) => {
                let query_str = r#"
                    delete from users where user_id = $1
                "#;

                let rows_affected = sqlx::query(query_str)
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
                    StatusCode::NO_CONTENT,
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
pub async fn delete_post(
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
                    delete from posts where id = $1 and user_id = $2
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
                    StatusCode::NO_CONTENT,
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
