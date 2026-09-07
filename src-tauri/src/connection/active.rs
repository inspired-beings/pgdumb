use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use super::credentials::CredentialStore;
use super::store::ConnectionStore;

pub struct ActiveConnection {
    pub profile_id: String,
    // Read by the query-execution feature, not yet built.
    #[allow(dead_code)]
    pub client: tokio_postgres::Client,
    pub driver_handle: JoinHandle<()>,
}

impl ActiveConnection {
    pub fn stop(self) {
        self.driver_handle.abort();
    }
}

pub struct AppState {
    pub connection_store: Arc<dyn ConnectionStore>,
    pub credential_store: Arc<dyn CredentialStore>,
    pub active_connection: Mutex<Option<ActiveConnection>>,
}

impl AppState {
    pub fn new(
        connection_store: Arc<dyn ConnectionStore>,
        credential_store: Arc<dyn CredentialStore>,
    ) -> Self {
        Self {
            connection_store,
            credential_store,
            active_connection: Mutex::new(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::credentials::MockCredentialStore;
    use crate::connection::store::JsonFileConnectionStore;

    #[tokio::test]
    async fn starts_with_no_active_connection() {
        let dir = tempfile::tempdir().unwrap();
        let state = AppState::new(
            Arc::new(JsonFileConnectionStore::new(dir.path().join("connections.json"))),
            Arc::new(MockCredentialStore::new()),
        );

        assert!(state.active_connection.lock().await.is_none());
    }
}
