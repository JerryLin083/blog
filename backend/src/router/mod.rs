use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};

use crate::handler::{err, greeting};

pub async fn router(state: Pool<Postgres>) -> Router {
    let app = Router::new()
        .route("/", get(greeting))
        .route("/post", post(err))
        .with_state(state);

    tracing::info!("Router init ...");

    app
}
