pub mod commands;
pub mod db;
pub mod logger;
pub mod services;
pub mod utils;

use crate::commands::*;
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
        .invoke_handler(tauri::generate_handler![
            // ===============================
            // Applications
            // ===============================
            create_application_command,
            update_application_command,
            get_application_by_id_command,
            get_all_applications_command,
            delete_application_command,
            // ===============================
            // Companies
            // ===============================
            create_company_command,
            get_company_by_id_command,
            get_all_companies_command,
            update_company_command,
            delete_company_command,
            // ===============================
            // Interactions
            // ===============================
            create_interaction_command,
            update_interaction_command,
            // ===============================
            // Job Listings
            // ===============================
            create_job_listing_command,
            get_job_listing_by_id_command,
            get_all_job_listings_command,
            update_job_listing_command,
            delete_job_listing_command,
            // ===============================
            // Notes
            // ===============================
            create_note_command,
            get_note_by_id_command,
            get_all_notes_command,
            update_note_command,
            delete_note_command,
            // ===============================
            // Persons
            // ===============================
            create_person_command,
            get_person_by_id_command,
            get_all_persons_command,
            update_person_command,
            delete_person_command,
            // ===============================
            // Reminders
            // ===============================
            create_reminder_command,
            get_reminder_by_id_command,
            get_all_reminders_command,
            update_reminder_command,
            delete_reminder_command
        ])
        // ============================================================
        // 4. Launch application
        // ============================================================
        .run(tauri::generate_context!())
        .expect("Error while running JobTrackr application");
}
