use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SslMode {
    Disable,
    Prefer,
    Require,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionProfile {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub ssl_mode: SslMode,
    pub remember_password: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ConnectionProfile {
        ConnectionProfile {
            id: "abc-123".to_string(),
            name: "Local".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            user: "postgres".to_string(),
            ssl_mode: SslMode::Prefer,
            remember_password: true,
        }
    }

    #[test]
    fn serializes_with_camel_case_keys() {
        let value = serde_json::to_value(sample()).unwrap();

        assert_eq!(value["id"], "abc-123");
        assert_eq!(value["sslMode"], "prefer");
        assert_eq!(value["rememberPassword"], true);
    }

    #[test]
    fn round_trips_through_json() {
        let json = serde_json::to_string(&sample()).unwrap();
        let parsed: ConnectionProfile = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed, sample());
    }
}
