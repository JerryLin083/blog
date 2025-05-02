use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use tower_http::services::ServeDir;

use crate::handler::home;

pub(crate) fn static_router() -> Router<Pool<Postgres>> {
    let router = Router::new().route("/home", get(home));

    router.nest_service("/assets", ServeDir::new("static/assets"))
}
