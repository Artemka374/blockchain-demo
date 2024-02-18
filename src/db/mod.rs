use crate::models::error::ServerError;
use crate::models::merkle_tree::MerkleProof;
use crate::models::primitives::{Address, Balance, Signature, H256};
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

pub async fn add_pending_transaction(
    conn: &mut PoolConn,
    tx: Transaction,
) -> Result<(), ServerError> {
    sqlx::query!(
        r#"
        INSERT INTO transactions (hash, from, to, amount, block_id, status)
        VALUES ($1, $2, $3, $4, 0, 'pending')
        "#,
        tx.hash.as_bytes(),
        tx.from,
        tx.to,
        tx.amount,
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed adding transaction: {}", e)))?;
    Ok(())
}

pub async fn add_transactions_to_block(
    conn: &mut PoolConn,
    tx_hashes: &[H256],
    block_id: u64,
) -> Result<(), ServerError> {
    sqlx::query!(
        r#"
        UPDATE transactions
        SET block_id = $1, status = 'confirmed'
        WHERE hash = ANY($2)
        "#,
        block_id,
        tx_hashes.iter().map(|h| h.as_bytes()).collect::<Vec<_>>()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed adding transaction to block: {}", e)))?;
    Ok(())
}

pub async fn get_balance(conn: &mut PoolConn, address: Address) -> Result<u64, ServerError> {
    ensure_address_exists(conn, address).await?;

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

pub async fn get_nonce(conn: &mut PoolConn, address: Address) -> Result<u64, ServerError> {
    ensure_address_exists(conn, address).await?;
    let nonce = sqlx::query!(
        r#"
        SELECT nonce
        FROM accounts
        WHERE address = $1
        "#,
        address.as_bytes()
    )
    .fetch_one(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting nonce: {}", e)))?
    .unwrap_or(0);

    Ok(nonce.result)
}

pub async fn ensure_address_exists(
    conn: &mut PoolConn,
    address: Address,
) -> Result<(), ServerError> {
    let _ = sqlx::query!(
        r#"
        INSERT INTO accounts (address, balance, nonce)
        VALUES ($1, 0, 0)
        ON CONFLICT (address) DO NOTHING
        "#,
        address.as_bytes()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed ensuring address exists: {}", e)))?;
    Ok(())
}

pub async fn update_balance(
    conn: &mut PoolConn,
    address: Address,
    amount: Balance,
) -> Result<(), ServerError> {
    ensure_address_exists(conn, address).await?;
    sqlx::query!(
        r#"
        UPDATE accounts
        SET balance = $1
        WHERE address = $2
        "#,
        amount,
        address.as_bytes()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed updating balance: {}", e)))?;
    Ok(())
}

pub async fn update_nonce(
    conn: &mut PoolConn,
    address: Address,
    nonce: u64,
) -> Result<(), ServerError> {
    ensure_address_exists(conn, address).await?;
    sqlx::query!(
        r#"
        UPDATE accounts
        SET nonce = $1
        WHERE address = $2
        "#,
        nonce,
        address.as_bytes()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed updating nonce: {}", e)))?;
    Ok(())
}
pub async fn get_transaction(
    conn: &mut PoolConn,
    tx_hash: H256,
) -> Result<Transaction, ServerError> {
    todo!()
}

pub async fn add_block(
    conn: &mut PoolConn,
    block_id: u64,
    hash: H256,
    parent_hash: H256,
    produced_by: Address,
) -> Result<(), ServerError> {
    // todo update query
    sqlx::query!(
        r#"
        INSERT INTO blocks (id, hash, parent_hash)
        VALUES ($1, $2)
        "#,
        block_id,
        hash.as_bytes()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed adding block: {}", e)))?;
    Ok(())
}

pub async fn get_pending_transactions(
    conn: &mut PoolConn,
    limit: u64,
) -> Result<Vec<Transaction>, ServerError> {
    let txs: Vec<_> = sqlx::query!(
        r#"
        SELECT hash, from, to, amount, sig, nonce
        FROM transactions
        WHERE status = 'pending'
        ORDER BY created_at
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting pending transactions: {}", e)))?;

    let txs = txs
        .into_iter()
        .map(|tx| Transaction {
            from: Address::from_slice(&tx.from).unwrap(),
            to: Address::from_slice(&tx.to).unwrap(),
            amount: tx.amount,
            nonce: tx.nonce,
            sig: Signature::from_slice(&tx.sig).unwrap(),
        })
        .collect();

    Ok(txs)
}

pub async fn get_latest_block(conn: &mut PoolConn) -> Result<(u64, H256), ServerError> {
    let result = sqlx::query!(
        r#"
        SELECT id, hash
        FROM blocks
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .fetch_one(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting latest block id: {}", e)))?
    .unwrap_or(0);

    Ok((result.id, H256::from_slice(&result.hash)))
}

pub async fn add_merkle_tree(conn: &mut PoolConn) -> Result<(), ServerError> {
    //todo: update method
    sqlx::query!(
        r#"
        INSERT INTO merkle_trees (block_id, root)
        VALUES ($1, $2)
        "#,
        block_id,
        root.as_bytes()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed adding merkle tree: {}", e)))?;
    Ok(())
}

pub async fn get_proof(conn: &mut PoolConn, tx_hash: H256) -> Result<MerkleProof, ServerError> {
    todo!()
}
