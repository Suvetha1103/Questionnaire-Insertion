use chrono::Utc;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn insert_user(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    let id = row.get("user_id")
        .ok_or_else(|| anyhow::anyhow!("Missing user_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid user_id"))?;

    sqlx::query!(
        "INSERT INTO \"USER\" (id, created_at, updated_at) VALUES ($1, $2, $3)",
        id,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    ).execute(pool).await?;
    Ok(())
}

