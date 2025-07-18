mod db;
mod excel;
mod insert;
mod schemas;

use anyhow::Result;
use db::get_db_pool;
use excel::read_excel;
use insert::{
    insert_parent_question,
    insert_question,
    insert_question_group,
    insert_questionnaire,
    insert_questionnaire_group_question,
    insert_questionnaire_group_version,
    insert_questionnaire_version,
    kit_questionnaire::insert_kit_questionnaire_versions,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::collections::HashMap;
use crate::schemas::questionnaire_version::QuestionnaireVersion;
use crate::schemas::questionnaire::Questionnaire;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let pool = get_db_pool().await?;
    test_connection(&pool).await?;

    let rows = read_excel("question_data_cleaned.xlsx")?;

    let mut group_name_to_id = HashMap::new();
    let mut next_group_id = 1;

    let mut group_version_to_id = HashMap::new();
    let mut next_group_version_id = 1;

    let mut inserted_group_versions = std::collections::HashSet::new();

    let mut successes = 0;
    let mut failures = vec![];

    for (i, row) in rows.iter().enumerate() {
        // Skip rows without a valid question_id
        if row.get("question_id").is_none() || row.get("question_id").unwrap().trim().is_empty() {
            continue;
        }
        
        let group_name = row.get("group_name").unwrap_or(&"UNKNOWN".to_string()).clone();
        let group_id = *group_name_to_id.entry(group_name.clone()).or_insert_with(|| {
            let id = next_group_id;
            next_group_id += 1;
            println!("Assigning group_id {} to group_name '{}'", id, group_name);
            id
        });

        // Generate questionnaire_group_version_id
        let group_version_key = group_name.clone();
        let questionnaire_group_version_id = *group_version_to_id.entry(group_version_key).or_insert_with(|| {
            let id = next_group_version_id;
            next_group_version_id += 1;
            id
        });

        let result = async {
            insert_question_group(&pool, row, group_id).await?;

            // Only insert if not already inserted
            if inserted_group_versions.insert(questionnaire_group_version_id) {
                insert_questionnaire_group_version(&pool, row, group_id, questionnaire_group_version_id).await?;
            }

            if row.get("question_id").is_some() {
                insert_question(&pool, row).await?;
            }
            insert_questionnaire_group_question(&pool, row).await?;
            insert_parent_question(&pool, row).await?;
            //insert_questionnaire(&pool, row).await?;
    
            Ok::<(), anyhow::Error>(())
        }.await;

        match result {
            Ok(_) => successes += 1,
            Err(e) => failures.push((i + 1, e.to_string())), // +1 for Excel row number (header is row 1)
        }
    }
    let version = QuestionnaireVersion {
        id: 1,
        description: "first version of questionnaire with 8 groups".to_string(),
        is_active: true,
    };

    insert_questionnaire_version(&pool, &version).await?;
    println!("✅ Import complete: {} succeeded, {} failed", successes, failures.len());
    if !failures.is_empty() {
        println!("❌ Failed rows:");
        for (row_num, err) in failures {
            println!("  Row {}: {}", row_num, err);
        }
    }
    let questionnaire = Questionnaire {
        id: 1, // or generate as needed
        questionnaire_group_version_id: 1,
        questionnaire_version_id: version.id,
        ordinal: 0, // or as needed
    };
    insert_questionnaire(&pool, &questionnaire).await?;
    insert_kit_questionnaire_versions(&pool, version.id).await?;

    Ok(())
}


// ✅ Simple DB connectivity check
async fn test_connection(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    println!("✅ Connected to PostgreSQL successfully.");
    Ok(())
}