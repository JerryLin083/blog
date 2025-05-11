use std::sync::Arc;

use axum::{
    Extension, Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use sqlx::{PgPool, Row};

use crate::session::SessionManager;

use super::{PostRequest, UserRequest};

pub async fn create_user(
    State(pool): State<PgPool>,
    Extension(mut session_manager): Extension<Arc<SessionManager>>,
    user: Json<UserRequest>,
) -> Result<Response, (StatusCode, String)> {
    let query_str = r#"
            insert into users(first_name, last_name, email, phone, address)
            values($1, $2, $3, $4, $5) returning user_id
        "#;

    let row = sqlx::query(query_str)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.address)
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let user_id: i32 = row.get(0);

    //create session
    let session_id = session_manager.create_session(user_id).await;

    //set session cookie
    let cookie_value = format!("session_id={}; HttpOnly; Path=/", session_id);

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    // 建立 redirect response
    let redirect_uri = format!("/users/{}", user_id);
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER) // 303 See Other
        .header("Location", redirect_uri)
        .header("Set-Cookie", cookie_value)
        .body(axum::body::Body::empty())
        .unwrap();

    Ok(response)
}

pub async fn create_post(
    State(pool): State<PgPool>,
    Extension(mut session_manager): Extension<Arc<SessionManager>>,
    headers: HeaderMap,
    post: Json<PostRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Get session id from header and check if in session_manager;
    if let Some(session_id_value) = headers.get("session_id") {
        if let Ok(session_id) = session_id_value.to_str() {
            if session_manager.check_session_id(session_id).await.is_none() {
                // Redirect to login page
                return Ok(Redirect::to("/login"));
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

    let query_str = r#"
        insert into posts(title, content, user_id)
        values($1, $2, $3)
        returning id, title
    "#;

    let row = sqlx::query(query_str)
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.user_id)
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let post_id: i32 = row.get(0);

    //redirect to post page
    Ok(Redirect::to(&format!("/posts/{}", post_id)))
}

//TODO: handle signup
pub async fn signup() {}

//TODO: handle login
pub async fn login() {}

//TODO: handle logout
pub async fn logout() {}
