use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::primitives::{Address, Id, Signature, H256};
use crate::models::Transaction;

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

pub async fn get_transaction(
    conn: &mut PoolConn,
    tx_hash: H256,
) -> Result<Transaction, ServerError> {
    todo!()
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
