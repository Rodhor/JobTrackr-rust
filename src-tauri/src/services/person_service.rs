use crate::db::models::enums::Role;
use crate::db::queries::person;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

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
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Person '{} {}' created successfully.", first_name, last_name),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!(
                "Error creating person '{} {}': {}",
                first_name, last_name, e
            );
            let json = serde_json::json!({
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
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Person {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving person {}: {}", id, e);
            let json = serde_json::json!({
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
            let json = serde_json::json!({
                "status": "success",
                "message": "All persons retrieved successfully.",
                "data": records
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving persons: {}", e);
            let json = serde_json::json!({
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
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Person {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating person {}: {}", id, e);
            let json = serde_json::json!({
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
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Person {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting person {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete person {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
