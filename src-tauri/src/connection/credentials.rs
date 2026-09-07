const SERVICE: &str = "com.inspired-beings.pgdumb";

pub trait CredentialStore: Send + Sync {
    fn get(&self, id: &str) -> Result<Option<String>, String>;
    fn set(&self, id: &str, password: &str) -> Result<(), String>;
    fn delete(&self, id: &str) -> Result<(), String>;
    // Read by the "remember password" checkbox availability check, not yet wired into the UI.
    #[allow(dead_code)]
    fn is_available(&self) -> bool;
}

pub struct KeyringCredentialStore;

impl CredentialStore for KeyringCredentialStore {
    fn get(&self, id: &str) -> Result<Option<String>, String> {
        let entry = keyring::Entry::new(SERVICE, id).map_err(|e| e.to_string())?;
        match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    fn set(&self, id: &str, password: &str) -> Result<(), String> {
        let entry = keyring::Entry::new(SERVICE, id).map_err(|e| e.to_string())?;
        entry.set_password(password).map_err(|e| e.to_string())
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let entry = keyring::Entry::new(SERVICE, id).map_err(|e| e.to_string())?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn is_available(&self) -> bool {
        keyring::Entry::store_status().is_ok()
    }
}

#[cfg(test)]
pub struct MockCredentialStore {
    passwords: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

#[cfg(test)]
impl MockCredentialStore {
    pub fn new() -> Self {
        Self {
            passwords: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[cfg(test)]
impl CredentialStore for MockCredentialStore {
    fn get(&self, id: &str) -> Result<Option<String>, String> {
        Ok(self.passwords.lock().unwrap().get(id).cloned())
    }

    fn set(&self, id: &str, password: &str) -> Result<(), String> {
        self.passwords
            .lock()
            .unwrap()
            .insert(id.to_string(), password.to_string());
        Ok(())
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        self.passwords.lock().unwrap().remove(id);
        Ok(())
    }

    fn is_available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_round_trips_a_password() {
        let store = MockCredentialStore::new();

        store.set("1", "hunter2").unwrap();

        assert_eq!(store.get("1").unwrap(), Some("hunter2".to_string()));
    }

    #[test]
    fn mock_returns_none_for_unknown_id() {
        let store = MockCredentialStore::new();

        assert_eq!(store.get("missing").unwrap(), None);
    }

    #[test]
    fn mock_delete_is_a_no_op_when_missing() {
        let store = MockCredentialStore::new();

        store.delete("missing").unwrap();
    }
}
