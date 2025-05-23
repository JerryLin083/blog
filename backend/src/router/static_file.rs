use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use tower_http::services::ServeDir;

use crate::handler::home;

pub(crate) fn static_router() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(home))
        .nest_service("/assets", ServeDir::new("static/assets"))
        .fallback(get(home))
}
