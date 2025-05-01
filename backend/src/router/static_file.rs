use axum::Router;
use sqlx::{Pool, Postgres};

pub(crate) fn static_router() -> Router<Pool<Postgres>> {
    //TODO:handle frontend file and MIME file.

    let router = Router::new();

    router
}
