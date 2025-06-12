use axum::{serve};
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::{info, error};
use tracing_subscriber;

mod handlers;
mod routes;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    // Initialize logger
    tracing_subscriber::fmt::init();

    let db_pool = PgPoolOptions::new()
        .connect("postgres://postgres:password@localhost:5432/orders_db")
        .await
        .unwrap_or_else(|e| {
            error!("DB connection failed: {}", e);
            std::process::exit(1);
        });

    let app = routes::order_routes(db_pool.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    info!("Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
