use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::{PgPool, Result, Row};

use super::{Post, User};

pub async fn users(State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rows = sqlx::query("select * from users order by user_id limit 10")
        .fetch_all(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let users: Vec<User> = rows
        .into_iter()
        .map(|row| User {
            user_id: row.get(0),
            first_name: row.try_get(1).unwrap_or("".to_string()),
            last_name: row.try_get(2).unwrap_or("".to_string()),
            email: row.try_get(3).unwrap_or("".to_string()),
            phone: row.try_get(4).unwrap_or("".to_string()),
            address: row.try_get(5).unwrap_or("".to_string()),
        })
        .collect();

    Ok(Json(users))
}

pub async fn posts(State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rows = sqlx::query(
        r#"
            select * from posts 
            where published_at is not null
            order by create_at desc limit 10 
        "#,
    )
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

    Ok(Json(posts))
}
