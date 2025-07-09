use sqlx::PgPool;
use std::collections::HashMap;
use chrono::Utc;

pub async fn insert_parent_question(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    let qid = row.get("question_id")
        .ok_or_else(|| anyhow::anyhow!("Missing question_id"))?;
    let pid = row.get("parent_question_id")
        .ok_or_else(|| anyhow::anyhow!("Missing parent_question_id"))?;

        sqlx::query!(
            "INSERT INTO parent_question (question_id, parent_question_id, created_at, updated_at)
             VALUES ($1, $2, $3, $4)",
            qid.parse::<i64>()?,
            pid.parse::<i64>()?,
            Utc::now().naive_utc(),
            Utc::now().naive_utc()
        ).execute(pool).await?;
    Ok(())
}
