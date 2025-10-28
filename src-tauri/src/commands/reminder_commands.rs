use crate::services::reminder_service;
use crate::services::service_types::JsonResult;
use chrono::NaiveDate;
use sqlx::SqlitePool;

// ======================================================
// Create Reminder Command
// ======================================================
#[tauri::command]
pub async fn create_reminder_command(
    pool: tauri::State<'_, SqlitePool>,
    application_id: Option<i64>,
    interaction_id: Option<i64>,
    note_id: Option<i64>,
    job_listing_id: Option<i64>,
    company_id: Option<i64>,
    person_id: Option<i64>,
    reminder_date: String,
    title: &str,
    message: Option<&str>,
    is_completed: bool,
) -> JsonResult {
    let parsed_date = match NaiveDate::parse_from_str(&reminder_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            let json = serde_json::json!({
                "status": "error",
                "message": "Invalid date format. Expected YYYY-MM-DD."
            });
            return Err(json.to_string());
        }
    };

    reminder_service::create_reminder_service(
        &pool,
        application_id,
        interaction_id,
        note_id,
        job_listing_id,
        company_id,
        person_id,
        &parsed_date,
        title,
        message,
        is_completed,
    )
    .await
}

// ======================================================
// Get Reminder by ID Command
// ======================================================
#[tauri::command]
pub async fn get_reminder_by_id_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    reminder_service::get_reminder_by_id_service(&pool, &id).await
}

// ======================================================
// Get All Reminders Command
// ======================================================
#[tauri::command]
pub async fn get_all_reminders_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    reminder_service::get_all_reminders_service(&pool).await
}

// ======================================================
// Update Reminder Command
// ======================================================
#[tauri::command]
pub async fn update_reminder_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    application_id: Option<i64>,
    interaction_id: Option<i64>,
    note_id: Option<i64>,
    job_listing_id: Option<i64>,
    company_id: Option<i64>,
    person_id: Option<i64>,
    reminder_date: Option<String>,
    title: Option<String>,
    message: Option<String>,
    is_completed: Option<bool>,
) -> JsonResult {
    let parsed_date = match reminder_date {
        Some(ref d) => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => {
                let json = serde_json::json!({
                    "status": "error",
                    "message": "Invalid date format. Expected YYYY-MM-DD."
                });
                return Err(json.to_string());
            }
        },
        None => None,
    };

    reminder_service::update_reminder_service(
        &pool,
        &id,
        application_id,
        interaction_id,
        note_id,
        job_listing_id,
        company_id,
        person_id,
        parsed_date.as_ref(),
        title.as_deref(),
        message.as_deref(),
        is_completed,
    )
    .await
}

// ======================================================
// Delete Reminder Command
// ======================================================
#[tauri::command]
pub async fn delete_reminder_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    reminder_service::delete_reminder_service(&pool, &id).await
}
