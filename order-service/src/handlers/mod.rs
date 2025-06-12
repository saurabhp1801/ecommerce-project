use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;

use crate::{
    errors::AppError,
    models::{CreateOrderPayload, Order, UpdateOrderPayload},
};

pub async fn create_order(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateOrderPayload>,
) -> Result<Json<Order>, AppError> {
    let order = sqlx::query_as!(
        Order,
        "INSERT INTO orders (user_id, product_id, quantity, status) 
         VALUES ($1, $2, $3, $4) RETURNING *",
        payload.user_id,
        payload.product_id,
        payload.quantity,
        payload.status
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(order))
}

pub async fn get_order(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Order>, AppError> {
    let order = sqlx::query_as!(Order, "SELECT * FROM orders WHERE id = $1", id)
        .fetch_optional(&pool)
        .await?;

    match order {
        Some(order) => Ok(Json(order)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_order(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateOrderPayload>,
) -> Result<Json<Order>, AppError> {
    let order = sqlx::query_as!(
        Order,
        "UPDATE orders SET 
            quantity = COALESCE($1, quantity),
            status = COALESCE($2, status)
         WHERE id = $3 RETURNING *",
        payload.quantity,
        payload.status,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match order {
        Some(order) => Ok(Json(order)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_order(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<&'static str>, AppError> {
    let result = sqlx::query!("DELETE FROM orders WHERE id = $1", id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(Json("Order deleted"))
    }
}

pub async fn list_orders(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Order>>, AppError> {
    let orders = sqlx::query_as!(Order, "SELECT * FROM orders")
        .fetch_all(&pool)
        .await?;

    Ok(Json(orders))
}
