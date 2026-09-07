use futures_util::StreamExt;
use serde::Serialize;
use tokio_postgres::{Client, SimpleQueryMessage};

use super::tag::{reconstruct_tag, split_statements};

pub const MAX_ROWS_PER_STATEMENT: usize = 10_000;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum StatementOutcome {
    Rows {
        columns: Vec<String>,
        rows: Vec<Vec<Option<String>>>,
        truncated: bool,
    },
    CommandTag {
        tag: String,
    },
    Error {
        message: String,
    },
}

struct InProgressRows {
    columns: Vec<String>,
    rows: Vec<Vec<Option<String>>>,
    truncated: bool,
}

struct OutcomeBuilder {
    outcomes: Vec<StatementOutcome>,
    in_progress: Option<InProgressRows>,
    statement_index: usize,
    row_cap: usize,
}

impl OutcomeBuilder {
    fn new(row_cap: usize) -> Self {
        Self {
            outcomes: Vec::new(),
            in_progress: None,
            statement_index: 0,
            row_cap,
        }
    }

    fn on_row_description(&mut self, columns: Vec<String>) {
        self.in_progress = Some(InProgressRows {
            columns,
            rows: Vec::new(),
            truncated: false,
        });
    }

    fn on_row(&mut self, values: Vec<Option<String>>) {
        if let Some(progress) = &mut self.in_progress {
            if progress.rows.len() < self.row_cap {
                progress.rows.push(values);
            } else {
                progress.truncated = true;
            }
        }
    }

    fn on_command_complete(&mut self, affected_rows: u64, statement_texts: &[String]) {
        match self.in_progress.take() {
            Some(progress) => {
                self.outcomes.push(StatementOutcome::Rows {
                    columns: progress.columns,
                    rows: progress.rows,
                    truncated: progress.truncated,
                });
            }
            None => {
                let statement = statement_texts
                    .get(self.statement_index)
                    .map(String::as_str)
                    .unwrap_or("");
                self.outcomes.push(StatementOutcome::CommandTag {
                    tag: reconstruct_tag(statement, affected_rows),
                });
            }
        }
        self.statement_index += 1;
    }

    fn on_error(&mut self, message: String) {
        self.outcomes.push(StatementOutcome::Error { message });
    }

    fn finish(self) -> Vec<StatementOutcome> {
        self.outcomes
    }
}

pub async fn execute(client: &Client, sql: &str) -> Result<Vec<StatementOutcome>, String> {
    let statement_texts = split_statements(sql);
    let mut builder = OutcomeBuilder::new(MAX_ROWS_PER_STATEMENT);

    let stream = client
        .simple_query_raw(sql)
        .await
        .map_err(|e| e.to_string())?;
    let mut stream = Box::pin(stream);

    while let Some(message) = stream.next().await {
        match message {
            Ok(SimpleQueryMessage::RowDescription(columns)) => {
                builder.on_row_description(columns.iter().map(|c| c.name().to_string()).collect());
            }
            Ok(SimpleQueryMessage::Row(row)) => {
                let mut values = Vec::with_capacity(row.len());
                for i in 0..row.len() {
                    let value = row.try_get(i).map_err(|e| e.to_string())?;
                    values.push(value.map(str::to_string));
                }
                builder.on_row(values);
            }
            Ok(SimpleQueryMessage::CommandComplete(count)) => {
                builder.on_command_complete(count, &statement_texts);
            }
            Ok(_) => {}
            Err(e) => {
                builder.on_error(e.to_string());
                break;
            }
        }
    }

    Ok(builder.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_description_then_rows_then_command_complete_produces_one_rows_outcome() {
        let mut builder = OutcomeBuilder::new(10);
        builder.on_row_description(vec!["id".to_string(), "name".to_string()]);
        builder.on_row(vec![Some("1".to_string()), Some("Alice".to_string())]);
        builder.on_row(vec![Some("2".to_string()), None]);
        builder.on_command_complete(2, &[]);

        assert_eq!(
            builder.finish(),
            vec![StatementOutcome::Rows {
                columns: vec!["id".to_string(), "name".to_string()],
                rows: vec![
                    vec![Some("1".to_string()), Some("Alice".to_string())],
                    vec![Some("2".to_string()), None],
                ],
                truncated: false,
            }]
        );
    }

    #[test]
    fn a_bare_command_complete_produces_a_command_tag_outcome() {
        let mut builder = OutcomeBuilder::new(10);
        builder.on_command_complete(3, &["UPDATE t SET x = 1".to_string()]);

        assert_eq!(
            builder.finish(),
            vec![StatementOutcome::CommandTag {
                tag: "UPDATE 3".to_string()
            }]
        );
    }

    #[test]
    fn rows_past_the_cap_are_counted_as_truncated_not_stored() {
        let mut builder = OutcomeBuilder::new(1);
        builder.on_row_description(vec!["n".to_string()]);
        builder.on_row(vec![Some("1".to_string())]);
        builder.on_row(vec![Some("2".to_string())]);
        builder.on_row(vec![Some("3".to_string())]);
        builder.on_command_complete(3, &[]);

        assert_eq!(
            builder.finish(),
            vec![StatementOutcome::Rows {
                columns: vec!["n".to_string()],
                rows: vec![vec![Some("1".to_string())]],
                truncated: true,
            }]
        );
    }

    #[test]
    fn an_error_is_recorded_as_its_own_outcome() {
        let mut builder = OutcomeBuilder::new(10);
        builder.on_command_complete(0, &["BEGIN".to_string()]);
        builder.on_error("syntax error at or near \"FOO\"".to_string());

        assert_eq!(
            builder.finish(),
            vec![
                StatementOutcome::CommandTag {
                    tag: "BEGIN".to_string()
                },
                StatementOutcome::Error {
                    message: "syntax error at or near \"FOO\"".to_string()
                },
            ]
        );
    }

    #[test]
    fn outcomes_serialize_with_a_camel_case_type_tag() {
        let rows = StatementOutcome::Rows {
            columns: vec!["id".to_string()],
            rows: vec![vec![Some("1".to_string())], vec![None]],
            truncated: true,
        };
        let tag = StatementOutcome::CommandTag {
            tag: "UPDATE 3".to_string(),
        };
        let error = StatementOutcome::Error {
            message: "boom".to_string(),
        };

        assert_eq!(
            serde_json::to_value(&rows).unwrap(),
            serde_json::json!({"type": "rows", "columns": ["id"], "rows": [["1"], [null]], "truncated": true})
        );
        assert_eq!(
            serde_json::to_value(&tag).unwrap(),
            serde_json::json!({"type": "commandTag", "tag": "UPDATE 3"})
        );
        assert_eq!(
            serde_json::to_value(&error).unwrap(),
            serde_json::json!({"type": "error", "message": "boom"})
        );
    }

    #[test]
    fn multiple_statements_advance_the_statement_index_in_order() {
        let mut builder = OutcomeBuilder::new(10);
        let statements = vec![
            "INSERT INTO t VALUES (1)".to_string(),
            "DELETE FROM t".to_string(),
        ];
        builder.on_command_complete(1, &statements);
        builder.on_command_complete(1, &statements);

        assert_eq!(
            builder.finish(),
            vec![
                StatementOutcome::CommandTag {
                    tag: "INSERT 0 1".to_string()
                },
                StatementOutcome::CommandTag {
                    tag: "DELETE 1".to_string()
                },
            ]
        );
    }
}
