use std::time::Duration;

use axum::{Extension, Router};
use sqlx::{Pool, Postgres};

mod api;
use api::api_router;

mod static_file;
use static_file::static_router;

use crate::session::session_builder;

pub async fn router(pool: Pool<Postgres>) -> Router {
    let api_router = api_router();
    let session_manager = session_builder(Duration::from_secs(5 * 60));
    let static_router = static_router();

    //run background session expired checker
    let session_manager_for_bg = session_manager.clone();
    session_manager_for_bg.run_check();

    let app = Router::new()
        .merge(static_router)
        .nest("/api", api_router)
        .layer(Extension(session_manager.clone()))
        .with_state(pool);

    tracing::info!("Router init ...");

    app
}
