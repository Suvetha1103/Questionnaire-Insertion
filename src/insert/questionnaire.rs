use sqlx::PgPool;
use std::collections::HashMap;

pub async fn insert_questionnaire(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO questionnaire (id, questionnaire_group_version_id, questionnaire_version_id, ordinal)
         VALUES ($1, $2, $3, $4)",
        row["questionnaire_id"].parse::<i64>()?,
        row["questionnaire_group_version_id"].parse::<i64>()?,
        row["questionnaire_version_id"].parse::<i64>()?,
        row["questionnaire_ordinal"].parse::<i32>()?
    ).execute(pool).await?;
    Ok(())
}
