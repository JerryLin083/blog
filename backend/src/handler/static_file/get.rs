use std::path::PathBuf;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn home() -> Result<impl IntoResponse, (StatusCode, String)> {
    //get frontend html;
    let html_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("static")
        .join("index.html");

    let home_html = tokio::fs::read_to_string(html_path)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    Ok(Html(home_html))
}

//TODO: should by handle by SPA frontend web pages
pub async fn not_found() -> Result<impl IntoResponse, (StatusCode, String)> {
    let html_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("static")
        .join("404.html");

    let not_found_html = tokio::fs::read_to_string(html_path)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, err.to_string()))?;

    Ok(Html(not_found_html))
}
