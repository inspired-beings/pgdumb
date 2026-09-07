use serde::Deserialize;
use tauri::State;
use uuid::Uuid;

use super::active::{ActiveConnection, AppState};
use super::credentials::CredentialStore;
use super::profile::{ConnectionProfile, SslMode};
use super::store::ConnectionStore;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveConnectionInput {
    pub id: Option<String>,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub ssl_mode: SslMode,
    pub remember_password: bool,
    pub password: Option<String>,
}

fn save_connection_logic(
    connection_store: &dyn ConnectionStore,
    credential_store: &dyn CredentialStore,
    input: SaveConnectionInput,
) -> Result<ConnectionProfile, String> {
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let profile = ConnectionProfile {
        id: id.clone(),
        name: input.name,
        host: input.host,
        port: input.port,
        database: input.database,
        user: input.user,
        ssl_mode: input.ssl_mode,
        remember_password: input.remember_password,
    };

    connection_store.save(&profile)?;

    if profile.remember_password {
        if let Some(password) = input.password {
            credential_store.set(&id, &password)?;
        }
    } else {
        credential_store.delete(&id)?;
    }

    Ok(profile)
}

fn resolve_password(
    profile: &ConnectionProfile,
    provided: Option<String>,
    credential_store: &dyn CredentialStore,
) -> Result<String, String> {
    if profile.remember_password {
        credential_store
            .get(&profile.id)?
            .ok_or_else(|| "no remembered password found for this connection; re-enter it".to_string())
    } else {
        provided.ok_or_else(|| "a password is required to connect".to_string())
    }
}

#[tauri::command]
pub async fn list_connections(state: State<'_, AppState>) -> Result<Vec<ConnectionProfile>, String> {
    state.connection_store.list()
}

#[tauri::command]
pub async fn save_connection(
    state: State<'_, AppState>,
    input: SaveConnectionInput,
) -> Result<ConnectionProfile, String> {
    save_connection_logic(
        state.connection_store.as_ref(),
        state.credential_store.as_ref(),
        input,
    )
}

#[tauri::command]
pub async fn delete_connection(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let mut active = state.active_connection.lock().await;
    if active.as_ref().map(|c| c.profile_id == id).unwrap_or(false)
        && let Some(connection) = active.take()
    {
        connection.stop();
    }
    drop(active);

    state.connection_store.delete(&id)?;
    state.credential_store.delete(&id)
}

#[tauri::command]
pub async fn connect(
    state: State<'_, AppState>,
    id: String,
    password: Option<String>,
) -> Result<ConnectionProfile, String> {
    let profile = state
        .connection_store
        .list()?
        .into_iter()
        .find(|p| p.id == id)
        .ok_or_else(|| "connection not found".to_string())?;

    let resolved_password = resolve_password(&profile, password, state.credential_store.as_ref())?;

    let mut config = tokio_postgres::Config::new();
    config
        .host(&profile.host)
        .port(profile.port)
        .user(&profile.user)
        .dbname(&profile.database)
        .password(&resolved_password)
        .ssl_mode(match profile.ssl_mode {
            SslMode::Disable => tokio_postgres::config::SslMode::Disable,
            SslMode::Prefer => tokio_postgres::config::SslMode::Prefer,
            SslMode::Require => tokio_postgres::config::SslMode::Require,
        });

    let tls = super::tls::connector()?;
    let (client, connection) = config.connect(tls).await.map_err(|e| e.to_string())?;
    let driver_handle = tokio::spawn(async move {
        let _ = connection.await;
    });

    let mut active = state.active_connection.lock().await;
    if let Some(old) = active.take() {
        old.stop();
    }
    *active = Some(ActiveConnection {
        profile_id: id,
        client,
        driver_handle,
    });

    Ok(profile)
}

#[tauri::command]
pub async fn disconnect(state: State<'_, AppState>) -> Result<(), String> {
    let mut active = state.active_connection.lock().await;
    if let Some(connection) = active.take() {
        connection.stop();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::credentials::MockCredentialStore;
    use crate::connection::store::JsonFileConnectionStore;

    fn input(id: Option<&str>, remember: bool, password: Option<&str>) -> SaveConnectionInput {
        SaveConnectionInput {
            id: id.map(str::to_string),
            name: "Local".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            user: "postgres".to_string(),
            ssl_mode: SslMode::Prefer,
            remember_password: remember,
            password: password.map(str::to_string),
        }
    }

    fn profile_with_remember(remember: bool) -> ConnectionProfile {
        ConnectionProfile {
            id: "1".to_string(),
            name: "Local".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            user: "postgres".to_string(),
            ssl_mode: SslMode::Prefer,
            remember_password: remember,
        }
    }

    #[test]
    fn saving_with_remember_stores_the_password() {
        let dir = tempfile::tempdir().unwrap();
        let connection_store = JsonFileConnectionStore::new(dir.path().join("connections.json"));
        let credential_store = MockCredentialStore::new();

        let profile = save_connection_logic(
            &connection_store,
            &credential_store,
            input(None, true, Some("secret")),
        )
        .unwrap();

        assert_eq!(credential_store.get(&profile.id).unwrap(), Some("secret".to_string()));
    }

    #[test]
    fn saving_without_remember_clears_any_stored_password() {
        let dir = tempfile::tempdir().unwrap();
        let connection_store = JsonFileConnectionStore::new(dir.path().join("connections.json"));
        let credential_store = MockCredentialStore::new();
        credential_store.set("existing-id", "old-secret").unwrap();

        let profile = save_connection_logic(
            &connection_store,
            &credential_store,
            input(Some("existing-id"), false, None),
        )
        .unwrap();

        assert_eq!(credential_store.get(&profile.id).unwrap(), None);
    }

    #[test]
    fn resolve_password_uses_the_keychain_when_remembered() {
        let credential_store = MockCredentialStore::new();
        credential_store.set("1", "stored-secret").unwrap();

        let resolved = resolve_password(&profile_with_remember(true), None, &credential_store).unwrap();

        assert_eq!(resolved, "stored-secret");
    }

    #[test]
    fn resolve_password_errors_when_remembered_but_missing_from_keychain() {
        let credential_store = MockCredentialStore::new();

        let result = resolve_password(&profile_with_remember(true), None, &credential_store);

        assert!(result.is_err());
    }

    #[test]
    fn resolve_password_uses_the_provided_value_when_not_remembered() {
        let credential_store = MockCredentialStore::new();

        let resolved = resolve_password(
            &profile_with_remember(false),
            Some("typed-in".to_string()),
            &credential_store,
        )
        .unwrap();

        assert_eq!(resolved, "typed-in");
    }

    #[test]
    fn resolve_password_errors_when_not_remembered_and_nothing_provided() {
        let credential_store = MockCredentialStore::new();

        let result = resolve_password(&profile_with_remember(false), None, &credential_store);

        assert!(result.is_err());
    }
}
