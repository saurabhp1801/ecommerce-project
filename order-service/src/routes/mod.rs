use axum::{Router, routing::{get, post, put, delete}};
use sqlx::PgPool;

use crate::handlers::*;

pub fn order_routes(db_pool: PgPool) -> Router {
    Router::new()
        .route("/orders", post(create_order).get(list_orders))
        .route("/orders/:id", get(get_order).put(update_order).delete(delete_order))
        .with_state(db_pool)
}
