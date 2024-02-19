use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::primitives::{Address, H256};
use crate::models::Transaction;

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
