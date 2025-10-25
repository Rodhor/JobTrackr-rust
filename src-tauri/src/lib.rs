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
            // APPLICATION COMMANDS
            // ===============================
            create_application_command,
            get_application_by_id_command,
            get_all_applications_command,
            update_application_command,
            delete_application_command,
            // ===============================
            // COMPANY COMMANDS
            // ===============================
            create_company_command,
            get_company_by_id_command,
            get_all_companies_command,
            update_company_command,
            delete_company_command,
            // ===============================
            // CONTACT COMMANDS
            // ===============================
            create_contact_command,
            get_contact_by_id_command,
            get_all_contacts_command,
            update_contact_command,
            delete_contact_command,
            // ===============================
            // CONTACT NOTE COMMANDS
            // ===============================
            create_contact_note_command,
            get_contact_note_by_id_command,
            get_all_contact_notes_command,
            update_contact_note_command,
            delete_contact_note_command,
            // ===============================
            // JOB LISTING COMMANDS
            // ===============================
            create_job_listing_command,
            get_job_listing_by_id_command,
            get_all_job_listings_command,
            update_job_listing_command,
            delete_job_listing_command,
            // ===============================
            // JOB LISTING CONTACT COMMANDS
            // ===============================
            create_job_listing_contact_command,
            get_job_listing_contact_by_id_command,
            get_all_job_listing_contacts_command,
            update_job_listing_contact_command,
            delete_job_listing_contact_command,
            // ===============================
            // PERSON COMMANDS
            // ===============================
            create_person_command,
            get_person_by_id_command,
            get_all_persons_command,
            update_person_command,
            delete_person_command,
            // ===============================
            // USER COMMANDS
            // ===============================
            create_user_command,
            update_user_command,
            delete_user_command
        ])
        // ============================================================
        // 4. Launch application
        // ============================================================
        .run(tauri::generate_context!())
        .expect("Error while running JobTrackr application");
}
