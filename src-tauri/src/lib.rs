mod connection;

use std::sync::Arc;

use tauri::Manager;

use connection::active::AppState;
use connection::credentials::KeyringCredentialStore;
use connection::store::JsonFileConnectionStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    connection::tls::install_crypto_provider();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let config_dir = app.path().app_config_dir()?;
            let connection_store = Arc::new(JsonFileConnectionStore::new(
                config_dir.join("connections.json"),
            ));
            let credential_store = Arc::new(KeyringCredentialStore);
            app.manage(AppState::new(connection_store, credential_store));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connection::commands::list_connections,
            connection::commands::save_connection,
            connection::commands::delete_connection,
            connection::commands::connect,
            connection::commands::test_connection,
            connection::commands::disconnect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
