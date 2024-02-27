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

    let nodes = tree.nodes;
    for i in 0..nodes.len() {
        add_merkle_node(conn, block_id, root, nodes[i].clone(), i).await?;
    }
    Ok(())
}

pub async fn add_merkle_node(
    conn: &mut PoolConn,
    block_id: Id,
    root: H256,
    node: MerkleNode,
    index: usize,
) -> Result<(), ServerError> {
    sqlx::query!(
        r#"
        INSERT INTO merkle_nodes (block_id, root, node, index)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (block_id, root, index) DO NOTHING
        "#,
        block_id as i64,
        root.as_bytes(),
        &node.as_bvtes()[..],
        index as i64
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed adding merkle node: {}", e)))?;

    Ok(())
}

pub async fn get_merkle_tree(conn: &mut PoolConn, block_id: Id) -> Result<MerkleTree, ServerError> {
    let rows: Vec<_> = sqlx::query!(
        r#"
        SELECT node, index
        FROM merkle_nodes
        WHERE block_id = $1
        ORDER BY index
        "#,
        block_id as i64
    )
    .fetch_all(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting transaction index: {}", e)))?;

    let nodes = rows
        .into_iter()
        .map(|row| {
            let mut buffer = [0u8; 33];
            buffer.copy_from_slice(&row.node.unwrap());
            MerkleNode::from_bytes(buffer).expect("Failed deserializing merkle node from database")
        })
        .collect::<Vec<_>>();

    MerkleTree::from_nodes(nodes).map_err(|e| e.into())
}

pub async fn get_transaction_index_and_block(
    conn: &mut PoolConn,
    tx_hash: H256,
) -> Result<Option<(Id, Id)>, ServerError> {
    let result = sqlx::query!(
        r#"
        SELECT index_in_block, block_id
        FROM transactions
        WHERE hash = $1
        "#,
        tx_hash.as_bytes()
    )
    .fetch_one(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting transaction index: {}", e)))?;

    if result.block_id.is_some() && result.index_in_block.is_some() {
        Ok(Some((
            result.index_in_block.unwrap() as Id,
            result.block_id.unwrap() as Id,
        )))
    } else {
        Ok(None)
    }
}
