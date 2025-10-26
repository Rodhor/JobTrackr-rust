pub mod commands;
pub mod db;
pub mod logger;
pub mod services;
pub mod utils;

use crate::db::connection::init_db;
use crate::logger::*;
use sqlx::SqlitePool;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    // ============================================================
    // 1. Initialize logger
    // ============================================================
    logger::init();
    info!("Logger initialized.");

    // ============================================================
    // 2. Initialize database connection pool
    // ============================================================
    let pool: SqlitePool = init_db().await;
    info!("Database initialized and ready.");

    // ============================================================
    // 3. Build Tauri app and register backend command handlers
    // ============================================================
    tauri::Builder::default()
        // --------------------------------------------------------
        // Plugins
        // --------------------------------------------------------
        .plugin(tauri_plugin_opener::init())
        // --------------------------------------------------------
        // Global managed state
        // --------------------------------------------------------
        .manage(pool)
        // --------------------------------------------------------
        // Command registration
        // --------------------------------------------------------
        .invoke_handler(tauri::generate_handler![])
        // ============================================================
        // 4. Launch application
        // ============================================================
        .run(tauri::generate_context!())
        .expect("Error while running JobTrackr application");
}
