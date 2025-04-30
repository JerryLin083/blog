use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use sqlx::{Pool, Postgres};

use crate::handler::{
    create_post, create_user, delete_post, delete_user, patch_post, patch_user, posts, users,
};

pub async fn router(state: Pool<Postgres>) -> Router {
    let get_router = Router::new()
        .route("/api/users", get(users))
        .route("/api/posts", get(posts));

    let post_router = Router::new()
        .route("/api/users", post(create_user))
        .route("/api/posts", post(create_post));

    let patch_router = Router::new()
        .route("/api/users/{id}", patch(patch_user))
        .route("/api/posts/{id}", patch(patch_post));

    let delete_router = Router::new()
        .route("/api/users/{id}", delete(delete_user))
        .route("/api/posts/{id}", delete(delete_post));

    let app = Router::new()
        .merge(get_router)
        .merge(post_router)
        .merge(patch_router)
        .merge(delete_router)
        .with_state(state);

    tracing::info!("Router init ...");

    app
}
