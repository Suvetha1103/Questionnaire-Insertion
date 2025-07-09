use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn get_db_pool() -> anyhow::Result<PgPool> {
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;
    Ok(pool)
}