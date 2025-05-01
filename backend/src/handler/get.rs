use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{PgPool, Result, Row};

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
        select * from posts where id = $1
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
        select * from posts 
        where published_at is not null
        order by create_at desc 
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
