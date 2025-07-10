use crate::schemas::questionnaire::Questionnaire;
use sqlx::PgPool;

pub async fn insert_questionnaire(pool: &PgPool, questionnaire: &Questionnaire) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO questionnaire (id, questionnaire_group_version_id, questionnaire_version_id, ordinal, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (id) DO NOTHING",
        questionnaire.id,
        questionnaire.questionnaire_group_version_id,
        questionnaire.questionnaire_version_id,
        questionnaire.ordinal,
        chrono::Utc::now().naive_utc(),
        chrono::Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;
    Ok(())
}
