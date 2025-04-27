use axum::http::StatusCode;
use sqlx::Result;

pub async fn greeting() -> Result<String, (StatusCode, String)> {
    let greeting = String::from("Hello world");

    Ok(greeting)
}
