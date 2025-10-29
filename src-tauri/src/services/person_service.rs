use crate::db::models::enums::Role;
use crate::db::queries::person;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use crate::services::service_utils::add_display_label;
use serde_json::{json, Value};
use sqlx::SqlitePool;

// ======================================================
// Helper: Format display label for person
// ======================================================
fn format_person_label(last_name: &str, first_name: &str, id: i64) -> String {
    let last_trimmed = last_name.trim();
    let first_trimmed = first_name.trim();

    if !last_trimmed.is_empty() && !first_trimmed.is_empty() {
        format!("{}, {}", last_trimmed, first_trimmed)
    } else if !last_trimmed.is_empty() {
        last_trimmed.to_string()
    } else if !first_trimmed.is_empty() {
        first_trimmed.to_string()
    } else {
        format!("Person ID: {}", id)
    }
}

// ======================================================
// Create Person
// ======================================================
pub async fn create_person_service(
    pool: &SqlitePool,
    first_name: &str,
    last_name: &str,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
    company_id: Option<i64>,
) -> JsonResult {
    info!("Creating person: {} {}", first_name, last_name);

    let result = person::create_person(
        pool,
        first_name,
        last_name,
        email,
        phone_number,
        role,
        linkedin_url,
        company_id,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Person created successfully. ID: {}", record.id);

            let display_label =
                format_person_label(&record.last_name, &record.first_name, record.id);
            let data = add_display_label(&record, Some(display_label));

            let json = json!({
                "status": "success",
                "message": format!("Person '{} {}' created successfully.", first_name, last_name),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!(
                "Error creating person '{} {}': {}",
                first_name, last_name, e
            );
            let json = json!({
                "status": "error",
                "message": format!("Failed to create person '{} {}': {}", first_name, last_name, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Person by ID
// ======================================================
pub async fn get_person_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving person ID: {}", id);

    let result = person::get_person_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            info!("Person retrieved successfully. ID: {}", id);

            let display_label =
                format_person_label(&record.last_name, &record.first_name, record.id);
            let data = add_display_label(&record, Some(display_label));

            let json = json!({
                "status": "success",
                "message": format!("Person {} retrieved successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving person {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve person {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Persons
// ======================================================
pub async fn get_all_persons_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all persons");

    let result = person::get_all_persons(pool).await;

    match result {
        Ok(records) => {
            info!("Persons retrieved successfully ({} total).", records.len());

            let data: Vec<Value> = records
                .into_iter()
                .map(|r| {
                    let display_label = format_person_label(&r.last_name, &r.first_name, r.id);
                    add_display_label(&r, Some(display_label))
                })
                .collect();

            let json = json!({
                "status": "success",
                "message": "All persons retrieved successfully.",
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving persons: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve persons: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Person
// ======================================================
pub async fn update_person_service(
    pool: &SqlitePool,
    id: &i64,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
    company_id: Option<i64>,
) -> JsonResult {
    info!("Updating person ID: {}", id);

    let result = person::update_person(
        pool,
        *id,
        first_name,
        last_name,
        email,
        phone_number,
        role,
        linkedin_url,
        company_id,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Person updated successfully. ID: {}", id);

            let display_label =
                format_person_label(&record.last_name, &record.first_name, record.id);
            let data = add_display_label(&record, Some(display_label));

            let json = json!({
                "status": "success",
                "message": format!("Person {} updated successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating person {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to update person {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Person
// ======================================================
pub async fn delete_person_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting person ID: {}", id);

    let result = person::delete_person(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Person deleted successfully. ID: {}", id);

            let json = json!({
                "status": "success",
                "message": format!("Person {} deleted successfully.", id)
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting person {}: {}", id, e);

            let json = json!({
                "status": "error",
                "message": format!("Failed to delete person {}: {}", id, e)
            });

            Err(json.to_string())
        }
    }
}
