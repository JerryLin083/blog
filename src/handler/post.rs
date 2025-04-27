use axum::http::StatusCode;

pub async fn err() -> Result<String, (StatusCode, String)> {
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from("Internal server error"),
    ))
}
