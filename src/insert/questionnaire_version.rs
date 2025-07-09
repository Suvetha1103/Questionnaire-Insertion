use sqlx::PgPool;
use std::collections::HashMap;

pub async fn insert_questionnaire_version(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO questionnaire_version (id, description, is_active)
         VALUES ($1, $2, $3)",
        row["questionnaire_version_id"].parse::<i64>()?,
        row.get("description"),
        row["is_active"].parse::<bool>()?
    ).execute(pool).await?;
    Ok(())
}
