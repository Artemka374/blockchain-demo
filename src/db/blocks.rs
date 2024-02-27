use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::primitives::{Address, Id, H256};
use crate::models::Block;

pub async fn add_block(
    conn: &mut PoolConn,
    block_id: u64,
    hash: H256,
    parent_hash: H256,
    produced_by: Address,
    nonce: u64,
) -> Result<(), ServerError> {
    // todo update query
    sqlx::query!(
        r#"
        INSERT INTO blocks (id, hash, parent_hash, produced_by, nonce)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        block_id as i64,
        hash.as_bytes(),
        parent_hash.as_bytes(),
        produced_by.as_bytes(),
        nonce as i64
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
    .fetch_optional(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting latest block id: {}", e)))?;

    match result {
        Some(block) => Ok((
            block.id.unwrap() as u64,
            H256::from_slice(&block.hash.unwrap()),
        )),
        None => Ok((0, H256::zero())),
    }
}

pub async fn get_block_by_id(
    conn: &mut PoolConn,
    block_id: Id,
) -> Result<Option<Block>, ServerError> {
    let result: Option<_> = sqlx::query!(
        r#"
        SELECT id, hash, parent_hash, merkle_root, produced_by, nonce
        FROM blocks
        WHERE id = $1
        "#,
        block_id as i64
    )
    .fetch_optional(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting block by id: {}", e)))?;

    let block = match result {
        Some(block) => block,
        None => return Ok(None),
    };

    Ok(Some(Block {
        id: block.id.unwrap() as Id,
        hash: block.hash.map(|hash| H256::from_slice(&hash)),
        parent_hash: H256::from_slice(&block.parent_hash.unwrap()),
        merkle_root: H256::from_slice(&block.merkle_root.unwrap()),
        produced_by: block.produced_by.map(|addr| Address::from_bytes(&addr)),
        nonce: block.nonce.map(|nonce| nonce as u64),
    }))
}

pub async fn get_block_by_hash(
    conn: &mut PoolConn,
    hash: H256,
) -> Result<Option<Block>, ServerError> {
    let result: Option<_> = sqlx::query!(
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

    let block = match result {
        Some(block) => block,
        None => return Ok(None),
    };

    Ok(Some(Block {
        id: block.id.unwrap() as Id,
        hash: block.hash.map(|hash| H256::from_slice(&hash)),
        parent_hash: H256::from_slice(&block.parent_hash.unwrap()),
        merkle_root: H256::from_slice(&block.merkle_root.unwrap()),
        produced_by: block.produced_by.map(|addr| Address::from_bytes(&addr)),
        nonce: block.nonce.map(|nonce| nonce as u64),
    }))
}
