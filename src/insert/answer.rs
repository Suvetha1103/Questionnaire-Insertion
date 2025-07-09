use sqlx::PgPool;
use std::collections::HashMap;
use chrono::Utc;

pub async fn insert_answer(pool: &PgPool, row: &HashMap<String, String>) -> anyhow::Result<()> {
    let id = row.get("answer_id")
        .ok_or_else(|| anyhow::anyhow!("Missing answer_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid answer_id"))?;
    let question_id = row.get("question_id")
        .ok_or_else(|| anyhow::anyhow!("Missing question_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid question_id"))?;
    let user_id = row.get("user_id")
        .ok_or_else(|| anyhow::anyhow!("Missing user_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid user_id"))?;
    let test_id = row.get("test_id")
        .ok_or_else(|| anyhow::anyhow!("Missing test_id"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("Invalid test_id"))?;

    sqlx::query!(
        "INSERT INTO answer (
            id, questionId, userId, answer, answerdate, kit_id, test_id,
            questionnaire_id, additional_answer, response_identifier
         ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        id,
        question_id,
        user_id,
        row.get("answer_value"),
        Utc::now().naive_utc(),
        row.get("kit_id"),
        test_id,
        row.get("questionnaire_id"),
        row.get("additional_answer"),
        row.get("response_identifier")
    ).execute(pool).await?;
    Ok(())
}
