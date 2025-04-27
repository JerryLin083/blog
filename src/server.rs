use tokio::net::TcpListener;

use crate::{db::db_connection, router::router};

pub async fn run() {
    let pool = db_connection().await;
    let router = router(pool).await;

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    tracing::info!("Listening on {}...", listener.local_addr().unwrap());

    if let Err(err) = axum::serve(listener, router).await {
        tracing::error!("Server error: {}", err);
    }
}
