use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use sqlx::{Pool, Postgres};

use crate::handler::{
    account, auth, create_post, delete_post, delete_user, edit_post, login, logout, patch_user,
    posts, publish_post, signup, the_post, the_user, users,
};

pub fn api_router() -> Router<Pool<Postgres>> {
    let get_router = Router::new()
        .route("/users", get(users))
        .route("/users/{id}", get(the_user))
        .route("/posts", get(posts))
        .route("/posts/{id}", get(the_post))
        .route("/account/{account}", get(account))
        .route("/auth", get(auth));

    let post_router = Router::new()
        .route("/singup", post(signup))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/posts", post(create_post));

    let patch_router = Router::new()
        .route("/users/{id}", patch(patch_user))
        .route("/posts/edit/{id}", patch(edit_post))
        .route("/posts/publish/{id}", patch(publish_post));

    let delete_router = Router::new()
        .route("/users/{id}", delete(delete_user))
        .route("/posts/{id}", delete(delete_post));

    let router = Router::new()
        .merge(get_router)
        .merge(post_router)
        .merge(patch_router)
        .merge(delete_router);

    router
}
