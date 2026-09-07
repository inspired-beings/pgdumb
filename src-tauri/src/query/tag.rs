const TWO_WORD_VERBS: &[&str] = &["CREATE", "DROP", "ALTER"];
const COUNTED_VERBS: &[&str] = &["UPDATE", "DELETE"];

pub fn split_statements(sql: &str) -> Vec<String> {
    let mut parts: Vec<String> = sql.split(';').map(|s| s.trim().to_string()).collect();
    if parts.last().map(|s| s.is_empty()).unwrap_or(false) {
        parts.pop();
    }
    parts
}

pub fn reconstruct_tag(statement: &str, affected_rows: u64) -> String {
    let tokens: Vec<String> = statement
        .split_whitespace()
        .take(2)
        .map(|t| t.to_uppercase())
        .collect();
    let verb = tokens.first().cloned().unwrap_or_default();

    let label = if TWO_WORD_VERBS.contains(&verb.as_str()) {
        match tokens.get(1) {
            Some(second) => format!("{verb} {second}"),
            None => verb.clone(),
        }
    } else {
        verb.clone()
    };

    if verb == "INSERT" {
        format!("INSERT 0 {affected_rows}")
    } else if COUNTED_VERBS.contains(&verb.as_str()) {
        format!("{label} {affected_rows}")
    } else {
        label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_on_semicolons_and_trims() {
        assert_eq!(
            split_statements("SELECT 1; SELECT 2 "),
            vec!["SELECT 1".to_string(), "SELECT 2".to_string()]
        );
    }

    #[test]
    fn drops_only_a_genuine_trailing_empty_segment() {
        assert_eq!(split_statements("SELECT 1;"), vec!["SELECT 1".to_string()]);
    }

    #[test]
    fn keeps_interior_empty_segments_from_a_double_semicolon() {
        assert_eq!(
            split_statements("SELECT 1;;SELECT 2;"),
            vec!["SELECT 1".to_string(), "".to_string(), "SELECT 2".to_string()]
        );
    }

    #[test]
    fn a_statement_with_no_trailing_semicolon_has_no_phantom_trailing_empty() {
        assert_eq!(split_statements("SELECT 1"), vec!["SELECT 1".to_string()]);
    }

    #[test]
    fn insert_gets_the_leading_zero_oid_form() {
        assert_eq!(reconstruct_tag("INSERT INTO t VALUES (1)", 1), "INSERT 0 1");
    }

    #[test]
    fn update_and_delete_get_a_plain_count() {
        assert_eq!(reconstruct_tag("UPDATE t SET x = 1", 3), "UPDATE 3");
        assert_eq!(reconstruct_tag("DELETE FROM t", 2), "DELETE 2");
    }

    #[test]
    fn create_drop_alter_keep_the_second_token_with_no_count() {
        assert_eq!(reconstruct_tag("CREATE TABLE t (id int)", 0), "CREATE TABLE");
        assert_eq!(reconstruct_tag("DROP INDEX idx_t", 0), "DROP INDEX");
        assert_eq!(reconstruct_tag("ALTER TABLE t ADD COLUMN y int", 0), "ALTER TABLE");
    }

    #[test]
    fn other_verbs_show_just_the_verb_with_no_count() {
        assert_eq!(reconstruct_tag("BEGIN", 0), "BEGIN");
        assert_eq!(reconstruct_tag("COMMIT", 0), "COMMIT");
    }

    #[test]
    fn an_empty_statement_produces_an_empty_tag() {
        assert_eq!(reconstruct_tag("", 0), "");
    }

    #[test]
    fn matching_is_case_insensitive_on_input_but_uppercases_the_verb() {
        assert_eq!(reconstruct_tag("insert into t values (1)", 1), "INSERT 0 1");
        assert_eq!(reconstruct_tag("create table t (id int)", 0), "CREATE TABLE");
    }
}
