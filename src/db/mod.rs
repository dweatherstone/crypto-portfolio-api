use sqlx::PgPool;

pub mod holding;
pub mod portfolio;
pub mod user;

pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
