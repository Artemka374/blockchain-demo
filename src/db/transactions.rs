use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::primitives::{Address, Balance, Id, H256};
use crate::models::{Transaction, TransactionStatus};

pub async fn add_pending_transaction(
    conn: &mut PoolConn,
    tx: Transaction,
) -> Result<(), ServerError> {
    sqlx::query!(
        r#"
        INSERT INTO transactions (hash, "from", "to", amount, block_id, status)
        VALUES ($1, $2, $3, $4, 0, 'pending')
        "#,
        tx.hash.as_bytes(),
        tx.from.as_bytes(),
        tx.to.as_bytes(),
        tx.amount as i64,
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
        block_id as i64,
        &tx_hashes
            .iter()
            .map(|h| h.as_bytes().to_vec())
            .collect::<Vec<_>>()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed adding transaction to block: {}", e)))?;
    Ok(())
}

pub async fn get_transaction(
    conn: &mut PoolConn,
    tx_hash: H256,
) -> Result<Option<Transaction>, ServerError> {
    let tx = sqlx::query!(
        r#"
        SELECT hash, "from", "to", amount, block_id, nonce, status
        FROM transactions
        WHERE hash = $1
        "#,
        tx_hash.as_bytes(),
    )
    .fetch_optional(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting transaction: {}", e)))?;

    let tx = match tx {
        Some(tx) => tx,
        None => return Ok(None),
    };

    Ok(Some(Transaction {
        hash: tx_hash,
        from: Address::from_bytes(&tx.from.unwrap()),
        to: Address::from_bytes(&tx.to.unwrap()),
        amount: tx.amount.unwrap() as Balance,
        block_id: tx.block_id.map(|id| id as Id),
        nonce: tx.nonce.unwrap() as u64,
        status: TransactionStatus::from(tx.status.unwrap().as_str()),
    }))
}

pub async fn get_transactions(
    conn: &mut PoolConn,
    address: Address,
) -> Result<Vec<Transaction>, ServerError> {
    let txs: Vec<_> = sqlx::query!(
        r#"
        SELECT hash, "from", "to", amount, block_id, nonce, status
        FROM transactions
        WHERE "from" = $1 OR "to" = $1
        "#,
        address.as_bytes()
    )
    .fetch_all(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting transactions: {}", e)))?;

    let txs = txs
        .into_iter()
        .map(|tx| Transaction {
            hash: H256::from_slice(&tx.hash.unwrap()),
            from: Address::from_bytes(&tx.from.unwrap()),
            to: Address::from_bytes(&tx.to.unwrap()),
            amount: tx.amount.unwrap() as Balance,
            block_id: tx.block_id.map(|id| id as Id),
            nonce: tx.nonce.unwrap() as u64,
            status: TransactionStatus::from(tx.status.unwrap().as_str()),
        })
        .collect();

    Ok(txs)
}

pub async fn get_pending_transactions(
    conn: &mut PoolConn,
    limit: u64,
) -> Result<Vec<Transaction>, ServerError> {
    let txs: Vec<_> = sqlx::query!(
        r#"
        SELECT hash, "from", "to", amount, nonce
        FROM transactions
        WHERE status = 'pending'
        ORDER BY timestamp
        LIMIT $1
        "#,
        limit as i64
    )
    .fetch_all(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed getting pending transactions: {}", e)))?;

    let txs = txs
        .into_iter()
        .map(|tx| Transaction {
            hash: H256::from_slice(&tx.hash.unwrap()),
            from: Address::from_bytes(&tx.from.unwrap()),
            to: Address::from_bytes(&tx.to.unwrap()),
            amount: tx.amount.unwrap() as Balance,
            block_id: None,
            nonce: tx.nonce.unwrap() as u64,
            status: TransactionStatus::Pending,
        })
        .collect();

    Ok(txs)
}
