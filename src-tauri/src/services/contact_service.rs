use crate::db::models::enums::ContactType;
use crate::db::queries::contact;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use chrono::NaiveDate;
use sqlx::SqlitePool;

// ======================================================
// Create Contact
// ======================================================
pub async fn create_contact_service(
    pool: &SqlitePool,
    contact_type: &ContactType,
    contact_date: &NaiveDate,
    location: Option<&str>,
    person_id: Option<i64>,
    application_id: Option<i64>,
) -> JsonResult {
    info!(
        "Creating contact (person_id={:?}, application_id={:?})",
        person_id, application_id
    );

    let result = contact::create_contact(
        pool,
        contact_type,
        contact_date,
        location,
        person_id,
        application_id,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Contact created successfully. ID: {}", record.id);
            let json = serde_json::json!({
                "status": "success",
                "message": "Contact created successfully.",
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating contact: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to create contact: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Contact by ID
// ======================================================
pub async fn get_contact_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving contact by ID: {}", id);

    let result = contact::get_contact_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            info!("Contact retrieved successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Contact {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving contact {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve contact {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Contacts
// ======================================================
pub async fn get_all_contacts_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all contacts");

    let result = contact::get_all_contacts(pool).await;

    match result {
        Ok(records) => {
            info!("Contacts retrieved successfully ({} total).", records.len());
            let json = serde_json::json!({
                "status": "success",
                "message": "All contacts retrieved successfully.",
                "data": records
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving contacts: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve contacts: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Contact
// ======================================================
pub async fn update_contact_service(
    pool: &SqlitePool,
    id: &i64,
    contact_type: Option<&ContactType>,
    contact_date: Option<&NaiveDate>,
    location: Option<&str>,
    person_id: Option<i64>,
    application_id: Option<i64>,
) -> JsonResult {
    info!("Updating contact ID: {}", id);

    let result = contact::update_contact(
        pool,
        *id,
        contact_type,
        contact_date,
        location,
        person_id,
        application_id,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Contact updated successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Contact {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating contact {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to update contact {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Contact
// ======================================================
pub async fn delete_contact_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting contact ID: {}", id);

    let result = contact::delete_contact(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Contact deleted successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Contact {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting contact {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete contact {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
