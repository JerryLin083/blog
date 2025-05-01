use axum::Router;
use sqlx::{Pool, Postgres};

mod api;
use api::api_router;

mod static_file;
use static_file::static_router;

pub async fn router(state: Pool<Postgres>) -> Router {
    let api_router = api_router();
    let static_router = static_router();

    let app = Router::new()
        .nest("/api", api_router)
        .nest("/", static_router)
        .with_state(state);

    tracing::info!("Router init ...");

    app
}
