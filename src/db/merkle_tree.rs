use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::merkle_tree::{MerkleNode, MerkleTree};
use crate::models::primitives::{Id, H256};

pub async fn add_merkle_tree(
    conn: &mut PoolConn,
    block_id: Id,
    tree: MerkleTree,
) -> Result<(), ServerError> {
    let root = tree.root().expect("Merkle tree has no nodes!");

    let nodes = tree
        .nodes
        .iter()
        .map(|node| node.as_bvtes())
        .collect::<Vec<_>>();
    let indexes = (0..tree.size).collect::<Vec<_>>();

    let query = format!(
        r"
        INSERT INTO merkle_trees (block_id, root, node, index)
        VALUES {}",
        indexes
            .iter()
            .map(|i| format!(
                "(${}, ${}, ${}, ${})",
                4 * i,
                4 * i + 1,
                4 * i + 2,
                4 * i + 3
            ))
            .collect::<Vec<_>>()
            .join(", "),
    );
    let mut query = sqlx::query(&query);

    for i in 0..tree.size {
        query = query
            .bind(block_id as i64)
            .bind(root.as_bytes())
            .bind(nodes[i])
            .bind(i as i64);
    }

    query
        .execute(conn)
        .await
        .map_err(|e| ServerError::new(500, format!("Failed adding merkle tree: {}", e)))?;

    Ok(())
}

pub async fn get_merkle_tree(conn: &mut PoolConn, block_id: Id) -> Result<MerkleTree, ServerError> {
    let mut nodes = Vec::new();

    let mut rows = sqlx::query!(
        r#"
        SELECT node, index
        FROM merkle_trees
        WHERE block_id = $1
        ORDER BY index
        "#,
        block_id
    )
    .fetch(conn);

    while let Some(row) = rows.next().await {
        let row =
            row.map_err(|e| ServerError::new(500, format!("Failed getting merkle tree: {}", e)))?;

        let node = MerkleNode::from_bytes(row.node)?;
        nodes.push(node);
    }

    MerkleTree::from_nodes(nodes).map_err(|e| e.into())
}

pub async fn get_transaction_index_and_block(
    conn: &mut PoolConn,
    tx_hash: H256,
) -> Result<Option<(Id, Id)>, ServerError> {
    let result = sqlx::query!(
        r#"
        SELECT index, block_id
        FROM transactions
        WHERE hash = $1
        "#,
        tx_hash.as_bytes()
    )
    .fetch_one(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting transaction index: {}", e)))?;

    Ok((result.index, result.block_id))
}
