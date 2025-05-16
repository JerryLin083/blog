use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

mod get;
pub use get::account;
pub use get::auth;
pub use get::logout;
pub use get::posts;
pub use get::the_post;
pub use get::the_user;
pub use get::users;

mod post;
pub use post::create_post;
pub use post::login;
pub use post::signup;

mod patch;
pub use patch::edit_post;
pub use patch::patch_user;
pub use patch::publish_post;

mod delete;
pub use delete::delete_post;
pub use delete::delete_user;

#[derive(Deserialize)]
pub(crate) struct UserRequest {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    address: String,
}

#[derive(Serialize)]
pub(crate) struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    address: String,
}

#[derive(Deserialize)]
pub(crate) struct PostRequest {
    title: String,
    content: String,
}

#[derive(Serialize)]
pub(crate) struct Post {
    id: i32,
    title: String,
    content: String,
    user_id: i32,
    author: String,
    create_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
    published_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub(crate) struct Account {
    account: String,
    password: String,
}

#[derive(Serialize)]
pub(crate) struct ApiErrorResponse {
    error: String,
    message: String,
}

impl ApiErrorResponse {
    pub fn from_internal_error(err: impl ToString) -> ApiErrorResponse {
        ApiErrorResponse {
            error: "Server internal error".into(),
            message: err.to_string(),
        }
    }

    pub fn from_bad_request(message: &str) -> ApiErrorResponse {
        ApiErrorResponse {
            error: "Bad request".into(),
            message: message.into(),
        }
    }

    pub fn from_unauthorized(message: &str) -> ApiErrorResponse {
        ApiErrorResponse {
            error: "Unauthorized".into(),
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct ApiResponse {
    status: String,
    result: String,
}

impl ApiResponse {
    pub fn from_ok(result: String) -> ApiResponse {
        ApiResponse {
            status: "Ok".into(),
            result,
        }
    }

    pub fn from_deny(result: String) -> ApiResponse {
        ApiResponse {
            status: "Deny".into(),
            result,
        }
    }
}
