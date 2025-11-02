pub mod commands;
pub mod db;
pub mod logger;
pub mod services;
pub mod utils;

use crate::commands::*;
use crate::db::connection::init_db;
use crate::logger::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Clone to produce an owned handle with 'static lifetime
            let app_handle = app.app_handle().clone();

            // Run async init in background to avoid blocking UI
            tauri::async_runtime::spawn(async move {
                // Logger (blocking)
                if let Err(e) = tokio::task::spawn_blocking(|| {
                    logger::init();
                    info!("Logger initialized.");
                })
                .await
                {
                    eprintln!("Logger initialization failed: {:?}", e);
                    return;
                }

                // Database (async)
                match init_db().await {
                    Ok(pool) => {
                        info!("Database initialized and ready.");
                        app_handle.manage(pool);
                    }
                    Err(e) => {
                        error!("Database initialization failed: {:?}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            handle_application_command,
            handle_company_command,
            handle_interaction_command,
            handle_job_listing_command,
            handle_note_command,
            handle_person_command,
            handle_reminder_command,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running JobTrackr application");
}
