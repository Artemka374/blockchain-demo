use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::primitives::{Address, Id, H256};
use crate::models::{Block, Transaction};

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

pub async fn get_block_by_id(
    conn: &mut PoolConn,
    block_id: Id,
) -> Result<Option<Block>, ServerError> {
    let result: Option<Block> = sqlx::query_as!(
        Block,
        r#"
        SELECT id, hash, parent_hash, merkle_root, produced_by, nonce, timestamp
        FROM blocks
        WHERE id = $1
        "#,
        block_id
    )
    .fetch_optional(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting block by id: {}", e)))?;

    Ok(result)
}

pub async fn get_block_by_hash(
    conn: &mut PoolConn,
    hash: H256,
) -> Result<Option<Block>, ServerError> {
    let result: Option<Block> = sqlx::query_as!(
        Block,
        r#"
        SELECT id, hash, parent_hash, merkle_root, produced_by, nonce, timestamp
        FROM blocks
        WHERE hash = $1
        "#,
        hash.as_bytes()
    )
    .fetch_optional(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting block by id: {}", e)))?;

    Ok(result)
}
