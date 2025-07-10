use sqlx::PgPool;
use chrono::Utc;

pub async fn insert_kit_questionnaire_versions(pool: &PgPool, questionnaire_version_id: i64) -> anyhow::Result<()> {
    let kit_types = vec!["GI", "FBI", "OHI"];
    for (i, kit_type) in kit_types.iter().enumerate() {
        let id = (i + 1) as i64;
        sqlx::query!(
            "INSERT INTO kit_questionnaire_version (id, kit_type, questionnaire_version_id, is_active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            id,
            kit_type,
            questionnaire_version_id,
            true,
            Utc::now().naive_utc(),
            Utc::now().naive_utc()
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}