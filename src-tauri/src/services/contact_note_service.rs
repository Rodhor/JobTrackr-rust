use crate::db::models::enums::NoteType;
use crate::db::queries::contact_note;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

pub async fn create_contact_note_service(
    pool: &SqlitePool,
    contact_id: i64,
    note_type: Option<&NoteType>,
    content: Option<&str>,
) -> JsonResult {
    info!("Creating contact note for contact_id: {}", contact_id);

    let result = contact_note::create_contact_note(pool, contact_id, note_type, content).await;

    match result {
        Ok(record) => {
            info!("Contact note created successfully. ID: {}", record.id);
            let json = serde_json::json!({
                "status": "success",
                "message": "Contact note created successfully.",
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating contact note: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to create contact note: {}", e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_contact_note_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving contact note ID: {}", id);

    let result = contact_note::get_contact_note_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Contact note {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving contact note: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve contact note {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_all_contact_notes_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all contact notes");

    let result = contact_note::get_all_contact_notes(pool).await;

    match result {
        Ok(records) => {
            info!("All contact notes retrieved.");
            let json = serde_json::json!({
                "status": "success",
                "message": "All contact notes retrieved successfully.",
                "data": records
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving contact notes: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve contact notes: {}", e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn update_contact_note_service(
    pool: &SqlitePool,
    id: &i64,
    contact_id: Option<i64>,
    note_type: Option<&NoteType>,
    content: Option<&str>,
) -> JsonResult {
    info!("Updating contact note ID: {}", id);

    let result = contact_note::update_contact_note(pool, *id, contact_id, note_type, content).await;

    match result {
        Ok(record) => {
            info!("Contact note updated. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Contact note {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating contact note: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to update contact note {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn delete_contact_note_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting contact note ID: {}", id);

    let result = contact_note::delete_contact_note(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Contact note deleted. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Contact note {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting contact note: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete contact note {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
