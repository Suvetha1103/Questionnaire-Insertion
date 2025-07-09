import pandas as pd
import json

df = pd.read_excel('question_data_cleaned.xlsx')

def is_json(val):
    try:
        if pd.isnull(val):
            return True
        json.loads(str(val))
        return True
    except Exception:
        return False

skipped = df[
    df['question_id'].isnull() |
    df['question_id'].duplicated() |
    ~df['is_required'].astype(str).str.lower().isin(['true','false']) |
    ~df['meta_data'].apply(is_json) |
    ~df['branch_on_parent_answer'].apply(is_json)
]

print('Rows that would be skipped:', len(skipped))
print(skipped[['question_id','is_required','meta_data','branch_on_parent_answer']])