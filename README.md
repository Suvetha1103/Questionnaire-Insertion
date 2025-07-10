**To clean the data:**
   python3 clean_excel.py

**To create tables:**
CREATE TABLE question (
    id BIGINT PRIMARY KEY,
    title TEXT,
    info TEXT,
    detailed_info TEXT,
    is_required BOOLEAN,
    has_single_parent_question BOOLEAN,
    branch_on_parent_answer JSONB,
    default_answer_if_hidden TEXT,
    answer_type TEXT,
    meta_data JSONB,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE question_group (
    id BIGINT PRIMARY KEY,
    name TEXT,
    image_url VARCHAR,
    description VARCHAR,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE questionnaire_group_version (
    id BIGSERIAL PRIMARY KEY,
    question_group_id BIGINT,  
    minutes_to_complete INT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE questionnaire_group_question (
    id BIGINT PRIMARY KEY,
    questionnaire_group_version_id BIGINT,  
    question_id BIGINT,                     
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE questionnaire_version (
    id BIGINT PRIMARY KEY,
    description TEXT,
    is_active BOOLEAN,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE questionnaire (
    id BIGINT PRIMARY KEY,
    questionnaire_group_version_id BIGINT,  
    questionnaire_version_id BIGINT,        
    ordinal INT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE parent_question (
    id BIGSERIAL PRIMARY KEY,
    question_id BIGINT,          
    parent_question_id BIGINT,    
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);



**To truncate table:**
  DELETE FROM questionnaire_group_question;
DELETE FROM questionnaire_group_version;
DELETE FROM question_group;
DELETE FROM question;
DELETE FROM questionnaire;
DELETE FROM questionnaire_version;
TRUNCATE TABLE parent_question;
