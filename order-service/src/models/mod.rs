use serde::{Deserialize, Serialize};

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateOrderPayload {
    pub user_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub status: String,
}

#[derive(Deserialize)]
pub struct UpdateOrderPayload {
    pub quantity: Option<i32>,
    pub status: Option<String>,
}
