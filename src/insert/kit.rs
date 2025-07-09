use sqlx::PgPool;
use std::collections::HashMap;
use chrono::Utc;
pub async fn insert_kit(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO kit (kitId, created_at, updated_at) VALUES ($1, $2, $3)",
        row["kit_id"].as_str(),
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    ).execute(pool).await?;
    Ok(())
}