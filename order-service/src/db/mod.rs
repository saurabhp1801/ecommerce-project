use sqlx::{Pool, Postgres};
use crate::config;

pub type DbPool = Pool<Postgres>;

pub async fn init() -> Result<DbPool, sqlx::Error> {
    let db_url = config::database_url();
    Pool::connect(&db_url).await
}
