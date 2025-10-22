pub mod db;
pub mod logger;

use crate::db::connection::init_db;
use crate::logger::*; // import macros directly
use sqlx::SqlitePool;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    logger::init(); // initialize first

    let pool: SqlitePool = init_db().await;
    info!("Database initialized and ready.");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("Error while running JobTrackr application");
}
