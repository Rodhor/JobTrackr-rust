use crate::db::queries::reminder;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use crate::services::service_utils::add_display_label;
use chrono::NaiveDate;
use serde_json::{json, Value};
use sqlx::SqlitePool;

// ======================================================
// Helper: Format display label for reminder
// ======================================================
fn format_reminder_label(title: &str, id: i64) -> String {
    let t = title.trim();
    if !t.is_empty() {
        t.to_string()
    } else {
        format!("Reminder ID: {}", id)
    }
}

// ======================================================
// Create Reminder
// ======================================================
pub async fn create_reminder_service(
    pool: &SqlitePool,
    application_id: Option<i64>,
    job_listing_id: Option<i64>,
    interaction_id: Option<i64>,
    note_id: Option<i64>,
    company_id: Option<i64>,
    person_id: Option<i64>,
    reminder_date: &NaiveDate,
    title: &str,
    message: Option<&str>,
    is_completed: bool,
) -> JsonResult {
    info!("Creating reminder: {:?}", title);

    let result = reminder::create_reminder(
        pool,
        application_id,
        job_listing_id,
        interaction_id,
        note_id,
        company_id,
        person_id,
        reminder_date,
        title,
        message,
        is_completed,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Reminder created successfully. ID: {}", record.id);

            let display_label = Some(format_reminder_label(&record.title, record.id));
            let data = add_display_label(&record, display_label);

            let json = json!({
                "status": "success",
                "message": format!("Reminder '{}' created successfully.", record.title),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating reminder: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to create reminder: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Reminder by ID
// ======================================================
pub async fn get_reminder_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving reminder ID: {}", id);

    let result = reminder::get_reminder_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            info!("Reminder retrieved successfully. ID: {}", id);

            let display_label = Some(format_reminder_label(&record.title, record.id));
            let data = add_display_label(&record, display_label);

            let json = json!({
                "status": "success",
                "message": format!("Reminder {} retrieved successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving reminder {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve reminder {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Reminders
// ======================================================
pub async fn get_all_reminders_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all reminders");

    let result = reminder::get_all_reminders(pool).await;

    match result {
        Ok(records) => {
            info!(
                "Reminders retrieved successfully ({} total).",
                records.len()
            );

            let data: Vec<Value> = records
                .into_iter()
                .map(|r| {
                    let display_label = Some(format_reminder_label(&r.title, r.id));
                    add_display_label(&r, display_label)
                })
                .collect();

            let json = json!({
                "status": "success",
                "message": "All reminders retrieved successfully.",
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving reminders: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve reminders: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Reminder
// ======================================================
pub async fn update_reminder_service(
    pool: &SqlitePool,
    id: &i64,
    application_id: Option<i64>,
    job_listing_id: Option<i64>,
    interaction_id: Option<i64>,
    note_id: Option<i64>,
    company_id: Option<i64>,
    person_id: Option<i64>,
    reminder_date: Option<&NaiveDate>,
    title: Option<&str>,
    message: Option<&str>,
    is_completed: Option<bool>,
) -> JsonResult {
    info!("Updating reminder ID: {}", id);

    let result = reminder::update_reminder(
        pool,
        *id,
        application_id,
        job_listing_id,
        interaction_id,
        note_id,
        company_id,
        person_id,
        reminder_date,
        title,
        message,
        is_completed,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Reminder updated successfully. ID: {}", id);

            let display_label = Some(format_reminder_label(&record.title, record.id));
            let data = add_display_label(&record, display_label);

            let json = json!({
                "status": "success",
                "message": format!("Reminder {} updated successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating reminder {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to update reminder {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Reminder
// ======================================================
pub async fn delete_reminder_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting reminder ID: {}", id);

    let result = reminder::delete_reminder(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Reminder deleted successfully. ID: {}", id);
            let json = json!({
                "status": "success",
                "message": format!("Reminder {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting reminder {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to delete reminder {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
