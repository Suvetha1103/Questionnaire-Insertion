use sqlx::PgPool;
use chrono::Utc;
use crate::schemas::questionnaire_version::QuestionnaireVersion;

pub async fn insert_questionnaire_version(pool: &PgPool, version: &QuestionnaireVersion) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO questionnaire_version (id, description, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (id) DO NOTHING",
        version.id,
        &version.description,
        version.is_active,
        Utc::now().naive_utc(),
        Utc::now().naive_utc()
    )
    .execute(pool)
    .await?;
    Ok(())
}
