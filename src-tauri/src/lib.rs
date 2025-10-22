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
    // Initialize logging before anything else
    logger::init();
    info!("Logger initialized.");

    // Initialize database
    let pool: SqlitePool = init_db().await;
    info!("Database initialized and ready.");

    // Build Tauri app with all command handlers
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error while running JobTrackr application");
}
