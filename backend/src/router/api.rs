use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use sqlx::{Pool, Postgres};

use crate::handler::{
    create_post, create_user, delete_post, delete_user, patch_post, patch_user, posts, the_post,
    the_user, users,
};

pub fn api_router() -> Router<Pool<Postgres>> {
    let get_router = Router::new()
        .route("/users", get(users))
        .route("/users/{id}", get(the_user))
        .route("/posts", get(posts))
        .route("/posts/{id}", get(the_post));

    let post_router = Router::new()
        .route("/users", post(create_user))
        .route("/posts", post(create_post));

    let patch_router = Router::new()
        .route("/users/{id}", patch(patch_user))
        .route("/posts/{id}", patch(patch_post));

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
