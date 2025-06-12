use axum::{serve};
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod handlers;
mod routes;
mod models;
mod errors;

#[tokio::main]
async fn main() {
    let db_pool = PgPoolOptions::new()
        .connect("postgres://postgres:password@localhost:5432/orders_db")
        .await
        .expect("DB connection failed");

    let app = routes::order_routes(db_pool.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
