use std::{collections::HashMap, sync::Arc};

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use sqlx::{PgPool, Result, Row};

use crate::session::SessionManager;

use super::{Post, User};

pub async fn the_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
        select * from users where user_id = $1
    "#;

    let rows = sqlx::query(query_str)
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

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
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(page) = params.get("page") {
        let query_str = r#"
        select * from users order by user_id limit 10 offset ($1::INT - 1)*10"#;

        let rows = sqlx::query(query_str)
            .bind(page)
            .fetch_all(&pool)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

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
            "Missing required query parameter: page".to_string(),
        ))
    }
}

pub async fn the_post(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

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
) -> Result<impl IntoResponse, (StatusCode, String)> {
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
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

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
            "Missing required query parameter: page".to_string(),
        ))
    }
}

pub async fn account(
    State(pool): State<PgPool>,
    Path(account): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let query_str = r#"
        select account_id from accounts where account = $1
    "#;

    let row = sqlx::query(query_str)
        .bind(&account)
        .fetch_all(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    if row.is_empty() {
        Ok((StatusCode::OK, "{\"result\": \"ok\"}"))
    } else {
        Ok((StatusCode::OK, "{\"result\": \"deny\"}"))
    }
}

pub async fn auth(
    headers: HeaderMap,
    Extension(session_manager): Extension<Arc<SessionManager>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(session_id_value) = headers.get("session_id") {
        if let Ok(session_id) = session_id_value.to_str() {
            match session_manager.check_auth(session_id).await {
                Some(_) => return Ok((StatusCode::OK, "Session ID authenticated".to_string())),
                None => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        "Session ID unauthenticated".to_string(),
                    ));
                }
            }
        } else {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Session ID Header".to_string(),
            ));
        }
    } else {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Authentication required.".to_string(),
        ));
    }
}
