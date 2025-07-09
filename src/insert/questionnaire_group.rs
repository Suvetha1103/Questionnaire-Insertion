use sqlx::PgPool;
use std::collections::HashMap;
use chrono::Utc;

pub async fn insert_question_group(
    pool: &PgPool,
    row: &HashMap<String, String>,
    group_id: i64
) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO question_group (id, name, image_url, description, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (id) DO NOTHING",
        group_id,
        row.get("question_group_name"),
        row.get("image_url"),
        row.get("description"),
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_questionnaire_group_version(
    pool: &PgPool,
    row: &HashMap<String, String>,
    group_id: i64,
    questionnaire_group_version_id: i64
) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO questionnaire_group_version (id, question_group_id, minutes_to_complete, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5)",
        questionnaire_group_version_id,
        group_id,
        row.get("minutes_to_complete").unwrap_or(&"0".to_string()).parse::<i32>()?,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    ).execute(pool).await?;
    Ok(())
}

pub async fn insert_questionnaire_group_question(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    let questionnaire_group_question_id = row.get("questionnaire_group_question_id")
        .ok_or_else(|| anyhow::anyhow!("Missing questionnaire_group_question_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid questionnaire_group_question_id"))?;
    let questionnaire_group_version_id = row.get("questionnaire_group_version_id")
        .ok_or_else(|| anyhow::anyhow!("Missing questionnaire_group_version_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid questionnaire_group_version_id"))?;
    let question_id = row.get("question_id")
        .ok_or_else(|| anyhow::anyhow!("Missing question_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid question_id"))?;
    let ordinal = row.get("ordinal")
        .ok_or_else(|| anyhow::anyhow!("Missing ordinal"))?
        .parse::<i32>()
        .map_err(|_| anyhow::anyhow!("Invalid ordinal"))?;
    sqlx::query!(
        "INSERT INTO questionnaire_group_question (id, questionnaire_group_version_id, question_id, ordinal, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        questionnaire_group_question_id,
        questionnaire_group_version_id,
        question_id,
        ordinal,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    ).execute(pool).await?;
    Ok(())
}
