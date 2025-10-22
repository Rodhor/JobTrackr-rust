use crate::db::models::enums::Role;
use crate::db::queries::person;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

pub async fn create_person_service(
    pool: &SqlitePool,
    first_name: &str,
    last_name: &str,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
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
    )
    .await;

    match result {
        Ok(record) => {
            info!("Person created. ID: {}", record.id);
            let json = serde_json::json!({
                "status": "success",
                "message": "Person created successfully.",
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating person: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to create person '{} {}': {}", first_name, last_name, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_person_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving person by ID: {}", id);

    let result = person::get_person_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Person {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving person: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve person {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_all_persons_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all persons");

    let result = person::get_all_persons(pool).await;

    match result {
        Ok(records) => {
            info!("All persons retrieved.");
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

pub async fn update_person_service(
    pool: &SqlitePool,
    id: &i64,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
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
    )
    .await;

    match result {
        Ok(record) => {
            info!("Person updated. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Person {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating person: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to update person {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn delete_person_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting person ID: {}", id);

    let result = person::delete_person(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Person deleted. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Person {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting person: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete person {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
