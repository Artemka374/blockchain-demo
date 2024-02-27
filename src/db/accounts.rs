use crate::db::PoolConn;
use crate::models::error::ServerError;
use crate::models::primitives::{Address, Balance};

pub async fn get_balance(conn: &mut PoolConn, address: Address) -> Result<Balance, ServerError> {
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
    .balance
    .unwrap_or(0);

    Ok(balance as Balance)
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
    .map_err(|e| ServerError::new(500, format!("Failed getting nonce: {}", e)))?;

    Ok(nonce.nonce.unwrap_or(0) as u64)
}

pub async fn ensure_address_exists(
    conn: &mut PoolConn,
    address: Address,
) -> Result<(), ServerError> {
    if !account_exists(conn, address).await? {
        let _ = sqlx::query!(
            r#"
            INSERT INTO accounts (address, balance, nonce)
            VALUES ($1, 0, 0)
            "#,
            address.as_bytes()
        )
        .execute(conn)
        .await
        .map_err(|e| ServerError::new(500, format!("Failed ensuring address exists: {}", e)))?;
    }
    Ok(())
}

async fn account_exists(conn: &mut PoolConn, address: Address) -> Result<bool, ServerError> {
    let result = sqlx::query!(
        r#"
        SELECT *
        FROM accounts
        WHERE address = $1
        "#,
        address.as_bytes()
    )
    .fetch_optional(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed ensuring address exists: {}", e)))?;

    Ok(result.is_some())
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
        amount as i64,
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
        nonce as i64,
        address.as_bytes()
    )
    .execute(conn)
    .await
    .map_err(|e| ServerError::new(500, format!("Failed updating nonce: {}", e)))?;
    Ok(())
}
