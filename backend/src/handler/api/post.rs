use axum::{
    Extension, Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use sqlx::{PgPool, Row};
use std::sync::Arc;

use super::{Account, ApiErrorResponse, ApiResponse, PostRequest};
use crate::session::SessionManager;

pub async fn create_post(
    State(pool): State<PgPool>,
    Extension(session_manager): Extension<Arc<SessionManager>>,
    jar: CookieJar,
    post: Json<PostRequest>,
) -> impl IntoResponse {
    // Get session id from cookie and check if in session_manager;
    if let Some(session_cookie) = jar.get("session_id") {
        let session_id = session_cookie.value();
        match session_manager.check_session_id(session_id).await {
            Some(session) => {
                let user_id_from_session = session.user_id;
                let query_str = r#"
                    insert into posts(title, content, user_id)
                    values($1, $2, $3)
                    returning id, title
                "#;

                let row = sqlx::query(query_str)
                    .bind(&post.title)
                    .bind(&post.content)
                    .bind(&user_id_from_session)
                    .fetch_one(&pool)
                    .await
                    .map_err(|err| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiErrorResponse::from_internal_error(err)),
                        )
                    })?;

                let post_id: i32 = row.get(0);

                //send new post id back to client
                Ok(Json(ApiResponse::from_ok(post_id.to_string())))
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

pub async fn signup(
    State(pool): State<PgPool>,
    Extension(mut session_manager): Extension<Arc<SessionManager>>,
    account: Json<Account>,
) -> Result<Response, (StatusCode, Json<ApiErrorResponse>)> {
    let query_str = r#"
        WITH new_account AS (
            insert into accounts(account, password)
            values($1, $2)
            returning account_id
        )

        insert into users(account_id)
        select account_id from new_account
        returning user_id
    "#;

    let row = sqlx::query(query_str)
        .bind(&account.account)
        .bind(&account.password)
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::from_internal_error(err)),
            )
        })?;

    let user_id: i32 = row.get(0);

    //create session
    let session_id = session_manager.create_session(user_id).await;

    //set session cookie
    let cookie_value = format!("session_id={}; HttpOnly; Path=/; Secure", session_id);

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    // 建立 redirect response
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER) // 303 See Other
        .header("Location", format!("/users/edit"))
        .header("Set-Cookie", cookie_value)
        .body(axum::body::Body::empty())
        .unwrap();

    Ok(response)
}

pub async fn login(
    State(pool): State<PgPool>,
    Extension(mut session_manager): Extension<Arc<SessionManager>>,
    account: Json<Account>,
) -> Result<Response, (StatusCode, Json<ApiErrorResponse>)> {
    let query_str = r#"
        select u.user_id from accounts a
        left join users u on u.account_id = a.account_id
        where a.account = $1 and a.password = $2
    "#;

    let row = sqlx::query(query_str)
        .bind(&account.account)
        .bind(&account.password)
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::from_internal_error(err)),
            )
        })?;

    let user_id: i32 = row.get(0);
    //create session
    let session_id = session_manager.create_session(user_id).await;

    //set session cookie
    let cookie_value = format!("session_id={}; HttpOnly; Path=/; Secure", session_id);

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", HeaderValue::from_str(&cookie_value).unwrap());

    // 建立 redirect response
    let response = Response::builder()
        .status(StatusCode::SEE_OTHER) // 303 See Other
        .header("Location", format!("/")) //to home page
        .header("Set-Cookie", cookie_value)
        .body(axum::body::Body::empty())
        .unwrap();
    Ok(response)
}
