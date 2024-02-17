use crate::models::error::ServerError;
use crate::models::merkle_tree::MerkleProof;
use crate::models::primitives::{Address, H256};
use crate::models::Transaction;
use serde::{Deserialize, Serialize};
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

pub async fn add_transaction(conn: &mut PoolConn, tx: Transaction) -> Result<(), ServerError> {
    sqlx::query!(
        r#"
        INSERT INTO transactions (from, to, amount, nonce, sig)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        tx.from,
        tx.to,
        tx.amount,
        tx.nonce,
        tx.sig
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed adding transaction: {}", e)))?;
    Ok(())
}

pub async fn get_balance(conn: &mut PoolConn, address: Address) -> Result<u64, ServerError> {
    let balance = sqlx::query!(
        r#"
        SELECT balance
        FROM accounts
        WHERE address = $1
        "#,
        address.as_bytes()
    )
    .fetch_one(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting balance: {}", e)))?
    .unwrap_or(0);

    Ok(balance.result)
}
pub async fn get_transaction(
    conn: &mut PoolConn,
    tx_hash: H256,
) -> Result<Transaction, ServerError> {
    todo!()
}

pub async fn get_proof(conn: &mut PoolConn, tx_hash: H256) -> Result<MerkleProof, ServerError> {
    todo!()
}
