use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use super::profile::ConnectionProfile;

pub trait ConnectionStore: Send + Sync {
    fn list(&self) -> Result<Vec<ConnectionProfile>, String>;
    fn save(&self, profile: &ConnectionProfile) -> Result<(), String>;
    fn delete(&self, id: &str) -> Result<(), String>;
}

pub struct JsonFileConnectionStore {
    file_path: PathBuf,
    lock: Mutex<()>,
}

impl JsonFileConnectionStore {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            lock: Mutex::new(()),
        }
    }

    fn read_all(&self) -> Result<Vec<ConnectionProfile>, String> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }
        let contents = fs::read_to_string(&self.file_path).map_err(|e| e.to_string())?;
        if contents.trim().is_empty() {
            return Ok(Vec::new());
        }
        serde_json::from_str(&contents).map_err(|e| e.to_string())
    }

    fn write_all(&self, profiles: &[ConnectionProfile]) -> Result<(), String> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let contents = serde_json::to_string_pretty(profiles).map_err(|e| e.to_string())?;
        fs::write(&self.file_path, contents).map_err(|e| e.to_string())
    }
}

impl ConnectionStore for JsonFileConnectionStore {
    fn list(&self) -> Result<Vec<ConnectionProfile>, String> {
        let _guard = self
            .lock
            .lock()
            .map_err(|_| "connection store lock poisoned".to_string())?;
        self.read_all()
    }

    fn save(&self, profile: &ConnectionProfile) -> Result<(), String> {
        let _guard = self
            .lock
            .lock()
            .map_err(|_| "connection store lock poisoned".to_string())?;
        let mut profiles = self.read_all()?;
        match profiles.iter_mut().find(|p| p.id == profile.id) {
            Some(existing) => *existing = profile.clone(),
            None => profiles.push(profile.clone()),
        }
        self.write_all(&profiles)
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let _guard = self
            .lock
            .lock()
            .map_err(|_| "connection store lock poisoned".to_string())?;
        let mut profiles = self.read_all()?;
        profiles.retain(|p| p.id != id);
        self.write_all(&profiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::profile::SslMode;

    fn sample(id: &str, name: &str) -> ConnectionProfile {
        ConnectionProfile {
            id: id.to_string(),
            name: name.to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            user: "postgres".to_string(),
            ssl_mode: SslMode::Prefer,
            remember_password: false,
        }
    }

    #[test]
    fn lists_empty_when_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonFileConnectionStore::new(dir.path().join("connections.json"));

        assert_eq!(store.list().unwrap(), Vec::new());
    }

    #[test]
    fn saves_and_lists_a_profile() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonFileConnectionStore::new(dir.path().join("connections.json"));

        store.save(&sample("1", "Local")).unwrap();

        assert_eq!(store.list().unwrap(), vec![sample("1", "Local")]);
    }

    #[test]
    fn saving_an_existing_id_updates_it_in_place() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonFileConnectionStore::new(dir.path().join("connections.json"));
        store.save(&sample("1", "Local")).unwrap();

        store.save(&sample("1", "Renamed")).unwrap();

        assert_eq!(store.list().unwrap(), vec![sample("1", "Renamed")]);
    }

    #[test]
    fn deletes_a_profile_by_id() {
        let dir = tempfile::tempdir().unwrap();
        let store = JsonFileConnectionStore::new(dir.path().join("connections.json"));
        store.save(&sample("1", "Local")).unwrap();
        store.save(&sample("2", "Other")).unwrap();

        store.delete("1").unwrap();

        assert_eq!(store.list().unwrap(), vec![sample("2", "Other")]);
    }
}
