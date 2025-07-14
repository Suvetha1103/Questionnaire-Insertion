use sqlx::PgPool;
use std::collections::HashMap;
use chrono::Utc;
use serde_json::Value as JsonValue;

pub async fn insert_question(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    let id = row.get("question_id")
        .ok_or_else(|| anyhow::anyhow!("Missing question_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid question_id"))?;
    let is_required = row.get("is_required")
        .ok_or_else(|| anyhow::anyhow!("Missing is_required"))?
        .parse::<bool>()
        .map_err(|_| anyhow::anyhow!("Invalid is_required"))?;
    let is_reset_question = row.get("is_reset_question")
        .ok_or_else(|| anyhow::anyhow!("Missing is_reset_question"))?
        .parse::<bool>()
        .map_err(|_| anyhow::anyhow!("Invalid is_reset_question"))?;
    let has_single_parent_question = match row.get("has_single_parent_question") {
        Some(v) => v.parse::<bool>().map_err(|_| anyhow::anyhow!("Invalid has_single_parent_question"))?,
        None => false,
    };

    let branch_on_parent_answer: JsonValue = serde_json::from_str(
        row.get("branch_on_parent_answer").map_or("null", |v| v)
    ).map_err(|_| anyhow::anyhow!("Invalid branch_on_parent_answer JSON"))?;

    let meta_data: JsonValue = serde_json::from_str(
        row.get("meta_data").map_or("null", |v| v)
    ).map_err(|_| anyhow::anyhow!("Invalid meta_data JSON"))?;

    sqlx::query!(
        r#"
        INSERT INTO question (
            id, title, info, detailed_info,
            is_required, is_reset_question, has_single_parent_question,
            branch_on_parent_answer, default_answer_if_hidden,
            answer_type, meta_data, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, $10, $11::jsonb, $12, $13)
        "#,
        id,
        row.get("title"),
        row.get("info"),
        row.get("detailed_info"),
        is_required,
        is_reset_question,
        has_single_parent_question,
        branch_on_parent_answer,
        row.get("default_answer_if_hidden"),
        row.get("answer_type"),
        meta_data,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;

    Ok(())
}