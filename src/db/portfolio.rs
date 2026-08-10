use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::state::{Holding, Portfolio, User};

#[derive(Debug, sqlx::FromRow)]
struct PortfolioRow {
    id: Uuid,
    user_id: Uuid,
    name: String,
    email: String,
}

#[derive(Debug, sqlx::FromRow)]
struct HoldingRow {
    id: Uuid,
    symbol: String,
    amount: Decimal,
}

pub async fn list(pool: &PgPool) -> Result<Vec<Portfolio>, sqlx::Error> {
    let rows = sqlx::query_as!(
        PortfolioRow,
        r#"
            SELECT p.id, p.user_id, p.name, u.email
            FROM portfolios p
            INNER JOIN users u ON u.id = p.user_id
            ORDER BY p.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let portfolios = rows
        .into_iter()
        .map(|row| Portfolio {
            id: row.id,
            name: row.name,
            user: User {
                id: row.user_id,
                email: row.email,
            },
            holdings: Vec::new(),
        })
        .collect();

    Ok(portfolios)
}

pub async fn get(pool: &PgPool, portfolio_id: Uuid) -> Result<Portfolio, sqlx::Error> {
    // Get the portfolio and its owner
    let portfolio = sqlx::query_as!(
        PortfolioRow,
        r#"
    SELECT p.id, p.user_id, p.name, u.email
    FROM portfolios p
    INNER JOIN users u ON p.user_id = u.id
    WHERE p.id = $1"#,
        portfolio_id
    )
    .fetch_one(pool)
    .await?;

    // Get the holdings belonging to this portfolio
    let holdings = sqlx::query_as!(
        HoldingRow,
        r#"
    SELECT id, symbol, amount
    FROM holdings
    WHERE portfolio_id = $1
    ORDER BY symbol"#,
        portfolio_id
    )
    .fetch_all(pool)
    .await?;

    // Turn the database rows into our application model
    let holdings = holdings
        .into_iter()
        .map(|row| Holding {
            id: row.id,
            symbol: row.symbol,
            amount: row.amount,
        })
        .collect();

    Ok(Portfolio {
        id: portfolio.id,
        name: portfolio.name,
        user: User {
            id: portfolio.user_id,
            email: portfolio.email,
        },
        holdings,
    })
}

pub async fn create(pool: &PgPool, user_id: Uuid, name: &str) -> Result<Uuid, sqlx::Error> {
    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO portfolios (user_id, name)
            VALUES ($1, $2)
            RETURNING id
        "#,
        user_id,
        name
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

pub async fn add_holdings(
    pool: &PgPool,
    portfolio_id: Uuid,
    holdings: &[(String, Decimal)],
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    for (symbol, amount) in holdings {
        sqlx::query!(
            r#"
            INSERT INTO holdings (portfolio_id, symbol, amount)
            VALUES ($1, $2, $3)
        "#,
            portfolio_id,
            symbol,
            amount
        )
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    Ok(())
}
