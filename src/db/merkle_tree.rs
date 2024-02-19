use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::merkle_tree::MerkleProof;
use crate::models::primitives::H256;

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
