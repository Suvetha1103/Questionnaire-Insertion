pub struct KitQuestionnaireVersion {
    pub id: i64,
    pub kit_type: String,
    pub questionnaire_version_id: i64,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}