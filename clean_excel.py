import pandas as pd
import json
import re

def fix_quotes(s):
    if pd.isna(s):
        return s
    # Replace curly quotes and single quotes with standard double quotes
    s = str(s)
    s = s.replace('“', '"').replace('”', '"').replace("‘", "'").replace("’", "'")
    s = re.sub(r"'", '"', s)
    return s

def fix_json_field(s):
    if pd.isna(s) or str(s).strip() == "" or str(s).strip().lower() == "null":
        return "null"  # Explicitly set as JSON null
    s = fix_quotes(s)
    try:
        obj = json.loads(s)
        return json.dumps(obj)
    except Exception:
        return "null"  # If invalid, set as JSON null

def ensure_int(val, default=None):
    try:
        return int(val)
    except Exception:
        return default


def main():
    df = pd.read_excel("question_data.xlsx", sheet_name="new table format", keep_default_na=False)
    # Clean JSON fields robustly
    for col in ["meta_data", "branch_on_parent_answer"]:
        if col in df.columns:
            df[col] = df[col].apply(fix_json_field)

    # Ensure question_id is int and not missing
    def ensure_int(val, default=None):
        try:
            return int(float(str(val).strip()))
        except Exception:
            print(f"Invalid question_id: {val}")  # Debug print
            return default

    # Ensure parent_question_id is the string 'null' if missing, empty, or 'null'
    if "parent_question_id" in df.columns:
        def fix_parent_id(x):
            if pd.isna(x):
                return 'null'
            s = str(x).strip().lower()
            if s == "" or s == "null":
                return 'null'
            return x
        df["parent_question_id"] = df["parent_question_id"].apply(fix_parent_id)

    # Generate unique questionnaire_group_question_id if missing
    if "questionnaire_group_question_id" not in df.columns:
        df["questionnaire_group_question_id"] = ""
    next_id = 1
    for idx, row in df.iterrows():
        if not df.at[idx, "questionnaire_group_question_id"]:
            df.at[idx, "questionnaire_group_question_id"] = next_id
            next_id += 1

    # Force all is_required values to lowercase strings before saving
    if "is_required" in df.columns:
        def to_bool_str(val):
            if pd.isna(val):
                return "false"
            val_str = str(val).strip().lower()
            if val_str in ["true", "1", "1.0", "yes", "y"]:
                return "true"
            if val_str in ["false", "0", "0.0", "no", "n"]:
                return "false"
            return "false"
        df["is_required"] = df["is_required"].apply(to_bool_str)

    # Assign a unique questionnaire_group_version_id for each group_name
    if "group_name" in df.columns:
        group_to_version_id = {}
        next_version_id = 1
        version_ids = []
        for group in df["group_name"]:
            if group not in group_to_version_id:
                group_to_version_id[group] = next_version_id
                next_version_id += 1
            version_ids.append(group_to_version_id[group])
        df["questionnaire_group_version_id"] = version_ids

    # Only keep rows where question_id is present (not empty or null)
    df = df[df["question_id"] != ""]

    # Force all ordinal values to integers before saving
    if "ordinal" in df.columns:
        def fix_ordinal(val):
            try:
                return int(val)
            except Exception:
                return 0  # Default value if empty or invalid
        df["ordinal"] = df["ordinal"].apply(fix_ordinal)
    if "default_answer_if_hidden" not in df.columns:
     df["default_answer_if_hidden"] = ""

    def fix_default_answer(val):
        val_str = str(val).strip()
        if val_str.lower() == "n/a":
            return "N/A"
        if val_str.lower() == "null":
            return "null"
        return val_str

    # Apply cleaning
    df["default_answer_if_hidden"] = df["default_answer_if_hidden"].apply(fix_default_answer)

    # Save and check
    print(df["default_answer_if_hidden"].unique())
    df.to_excel("question_data_cleaned.xlsx", index=False)
    print("Cleaned data saved to question_data_cleaned.xlsx")

   

if __name__ == "__main__":
    main()
