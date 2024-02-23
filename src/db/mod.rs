pub mod accounts;
pub mod blocks;
pub mod merkle_tree;
pub mod transactions;

use crate::models::error::ServerError;
use sqlx;
use sqlx::postgres::PgPool;
use std::env;

pub type PoolConn = sqlx::pool::PoolConnection<sqlx::Postgres>;

pub async fn connection(pool: &PgPool) -> Result<PoolConn, ServerError> {
    PgPool::acquire(pool)
        .await
        .map_err(|e| ServerError::new(500, format!("Failed getting database connection: {}", e)))
}

pub async fn init() -> Result<PgPool, ServerError> {
    let db_url = env::var("DATABASE_URL").expect("Database url must be set!");
    let pool = PgPool::connect(&db_url).await.unwrap();
    Ok(pool)
}
