use sqlx::PgPool;
use std::collections::HashMap;
use chrono::Utc;

pub async fn insert_test(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    let id = row.get("test_id")
        .ok_or_else(|| anyhow::anyhow!("Missing test_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid test_id"))?;

    sqlx::query!(
        "INSERT INTO test (id, created_at, updated_at) VALUES ($1, $2, $3)",
        id,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    ).execute(pool).await?;
    Ok(())
}