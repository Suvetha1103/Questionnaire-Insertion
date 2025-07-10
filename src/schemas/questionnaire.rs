// src/schemas/questionnaire.rs
pub struct Questionnaire {
    pub id: i64,
    pub questionnaire_group_version_id: i64,
    pub questionnaire_version_id: i64,
    pub ordinal: i32,
}