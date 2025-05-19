use std::{collections::HashMap, sync::Arc};

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use sqlx::{PgPool, Result, Row};

use crate::session::SessionManager;

use super::{ApiErrorResponse, ApiResponse, Post, User};

pub async fn the_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    let query_str = r#"
        select * from users where user_id = $1
    "#;

    let rows = sqlx::query(query_str)
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::from_internal_error(err)),
            )
        })?;

    let users: Vec<User> = rows
        .into_iter()
        .map(|row| User {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            email: row.get(3),
            phone: row.get(4),
            address: row.get(5),
        })
        .collect();

    Ok((StatusCode::OK, Json(users)))
}

pub async fn users(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(page) = params.get("page") {
        let query_str = r#"
            select 
                user_id, 
                COALESCE(first_name, ''),
                COALESCE(last_name, ''),
                COALESCE(email, ''), 
                COALESCE(phone, ''), 
                COALESCE(address, '')   
            from users 
            order by user_id 
            limit 10 offset ($1::INT - 1)*10
            "#;

        let rows = sqlx::query(query_str)
            .bind(page)
            .fetch_all(&pool)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::from_internal_error(err)),
                )
            })?;

        let users: Vec<User> = rows
            .into_iter()
            .map(|row| User {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2),
                email: row.get(3),
                phone: row.get(4),
                address: row.get(5),
            })
            .collect();

        Ok((StatusCode::OK, Json(users)))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse::from_bad_request(
                "Missing required query parameter: page",
            )),
        ))
    }
}

pub async fn the_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    let query_str = r#"
        select p.*, u.first_name || ' ' || u.last_name as author
        from posts p
        left join users u on u.user_id = p.user_id
        where p.id = $1
    "#;

    let rows = sqlx::query(query_str)
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::from_internal_error(err)),
            )
        })?;

    let posts: Vec<Post> = rows
        .into_iter()
        .map(|row| Post {
            id: row.get(0),
            title: row.get(1),
            content: row.get(2),
            user_id: row.get(3),
            create_at: row.get(4),
            update_at: row.get(5),
            published_at: row.get(6),
            author: row.try_get(7).unwrap_or("".to_string()),
        })
        .collect();

    Ok((StatusCode::OK, Json(posts)))
}

pub async fn posts(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(page) = params.get("page") {
        let query_str = r#"
        select p.*, u.first_name || ' ' || u.last_name as author
        from posts p
        left join users u on u.user_id = p.user_id
        where p.published_at is not null
        order by p.create_at desc 
        limit 10 offset ($1::INT - 1) * 10
    "#;

        let rows = sqlx::query(query_str)
            .bind(page)
            .fetch_all(&pool)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::from_internal_error(err)),
                )
            })?;

        let posts: Vec<Post> = rows
            .into_iter()
            .map(|row| Post {
                id: row.get(0),
                title: row.get(1),
                content: row.get(2),
                user_id: row.get(3),
                create_at: row.get(4),
                update_at: row.get(5),
                published_at: row.get(6),
                author: row.try_get(7).unwrap_or("".to_string()),
            })
            .collect();

        Ok((StatusCode::OK, Json(posts)))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(ApiErrorResponse::from_bad_request(
                "Missing required query parameter: page",
            )),
        ))
    }
}

//Confirm whether the account has been registered
pub async fn account(
    State(pool): State<PgPool>,
    Path(account): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    let query_str = r#"
        select account_id from accounts where account = $1
    "#;

    let row = sqlx::query(query_str)
        .bind(&account)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::from_internal_error(err)),
            )
        })?;

    if row.is_empty() {
        Ok((StatusCode::OK, Json(ApiResponse::from_ok("1".into()))))
    } else {
        Ok((StatusCode::OK, Json(ApiResponse::from_deny("0".into()))))
    }
}

pub async fn auth(
    jar: CookieJar,
    Extension(session_manager): Extension<Arc<SessionManager>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match session_manager.check_auth(session_id).await {
            Some(_) => {
                return Ok((
                    StatusCode::OK,
                    Json(ApiResponse::from_ok("session authenticated".into())),
                ));
            }
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiErrorResponse::from_unauthorized(
                        "session unauthenticated",
                    )),
                ));
            }
        }
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorResponse::from_unauthorized(
                "Invalid Session ID Header",
            )),
        ));
    }
}

pub async fn logout(
    Extension(session_manager): Extension<Arc<SessionManager>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();

        match session_manager.delete_session(session_id).await {
            Some(_) => {
                return Ok((
                    StatusCode::OK,
                    Json(ApiResponse::from_ok("User has been logout".into())),
                ));
            }
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiErrorResponse::from_unauthorized(
                        "session unauthenticated",
                    )),
                ));
            }
        }
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorResponse::from_unauthorized(
                "Invalid Session ID Header",
            )),
        ));
    }
}
