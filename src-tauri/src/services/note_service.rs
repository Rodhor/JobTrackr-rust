use crate::db::models::enums::NoteType;
use crate::db::queries::note;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use crate::services::service_utils::add_display_label;
use serde_json::{json, Value};
use sqlx::SqlitePool;

// ======================================================
// Create Note
// ======================================================
pub async fn create_note_service(
    pool: &SqlitePool,
    interaction_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<&NoteType>,
    title: Option<&str>,
    content: Option<&str>,
) -> JsonResult {
    info!(
        "Creating note (interaction_id={:?}, job_listing_id={:?}, application_id={:?})",
        interaction_id, job_listing_id, application_id
    );

    let result = note::create_note(
        pool,
        interaction_id,
        job_listing_id,
        application_id,
        person_id,
        company_id,
        note_type,
        title,
        content,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Note created successfully. ID: {}", record.id);
            let data = add_display_label(&record, record.title.as_deref());

            let json = json!({
                "status": "success",
                "message": "Note created successfully.",
                "data": data
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating note: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to create note: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Note by ID
// ======================================================
pub async fn get_note_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving note ID: {}", id);

    let result = note::get_note_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            let data = add_display_label(&record, record.title.as_deref());
            let json = json!({
                "status": "success",
                "message": format!("Note {} retrieved successfully.", id),
                "data": data
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving note {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve note {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Notes
// ======================================================
pub async fn get_all_notes_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all notes");

    let result = note::get_all_notes(pool).await;

    match result {
        Ok(records) => {
            info!("Notes retrieved successfully ({} total).", records.len());

            let data: Vec<Value> = records
                .into_iter()
                .map(|r| add_display_label(&r, r.title.as_deref()))
                .collect();

            let json = json!({
                "status": "success",
                "message": "All notes retrieved successfully.",
                "data": data
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving notes: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve notes: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Note
// ======================================================
pub async fn update_note_service(
    pool: &SqlitePool,
    id: &i64,
    interaction_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<&NoteType>,
    title: Option<&str>,
    content: Option<&str>,
) -> JsonResult {
    info!("Updating note ID: {}", id);

    let result = note::update_note(
        pool,
        *id,
        interaction_id,
        job_listing_id,
        application_id,
        person_id,
        company_id,
        note_type,
        title,
        content,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Note updated successfully. ID: {}", id);
            let data = add_display_label(&record, record.title.as_deref());
            let json = json!({
                "status": "success",
                "message": format!("Note {} updated successfully.", id),
                "data": data
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating note {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to update note {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Note
// ======================================================
pub async fn delete_note_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting note ID: {}", id);

    let result = note::delete_note(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Note deleted successfully. ID: {}", id);
            let json = json!({
                "status": "success",
                "message": format!("Note {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting note {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to delete note {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
