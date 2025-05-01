use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod get;
pub use get::posts;
pub use get::the_post;
pub use get::the_user;
pub use get::users;

mod post;
pub use post::create_post;
pub use post::create_user;

mod patch;
pub use patch::patch_post;
pub use patch::patch_user;

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
    user_id: i32,
}

#[derive(Serialize)]
pub(crate) struct Post {
    id: i32,
    title: String,
    content: String,
    user_id: i32,
    create_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
    published_at: DateTime<Utc>,
}
