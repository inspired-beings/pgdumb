use tauri::State;

use crate::connection::active::AppState;

use super::execute::{execute, StatementOutcome};

#[tauri::command]
pub async fn execute_query(
    state: State<'_, AppState>,
    sql: String,
) -> Result<Vec<StatementOutcome>, String> {
    let active = state.active_connection.lock().await;
    let connection = active
        .as_ref()
        .ok_or_else(|| "not connected".to_string())?;
    execute(&connection.client, &sql).await
}
