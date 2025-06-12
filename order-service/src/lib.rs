use sqlx::postgres::PgPoolOptions;



async fn prepare_queries(pool: sqlx::PgPool) -> sqlx::Result<()> {
    // Example query that mirrors your real queries
    let _ = sqlx::query!("SELECT * FROM orders").fetch_all(&pool).await?;
    Ok(())
}
