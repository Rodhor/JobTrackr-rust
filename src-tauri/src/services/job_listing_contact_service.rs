use crate::db::models::enums::Role;
use crate::db::queries::job_listing_contact;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

pub async fn create_job_listing_contact_service(
    pool: &SqlitePool,
    job_listing_id: i64,
    person_id: i64,
    role: Option<&Role>,
) -> JsonResult {
    info!(
        "Creating job listing contact for listing_id: {}",
        job_listing_id
    );

    let result =
        job_listing_contact::create_job_listing_contact(pool, job_listing_id, person_id, role)
            .await;

    match result {
        Ok(record) => {
            info!("Job listing contact created. ID: {}", record.id);
            let json = serde_json::json!({
                "status": "success",
                "message": "Job listing contact created successfully.",
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating job listing contact: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to create job listing contact: {}", e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_job_listing_contact_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving job listing contact ID: {}", id);

    let result = job_listing_contact::get_job_listing_contact_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Job listing contact {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving job listing contact: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve job listing contact {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_all_job_listing_contacts_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all job listing contacts");

    let result = job_listing_contact::get_all_job_listing_contacts(pool).await;

    match result {
        Ok(records) => {
            info!("Job listing contacts retrieved successfully.");
            let json = serde_json::json!({
                "status": "success",
                "message": "All job listing contacts retrieved successfully.",
                "data": records
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving job listing contacts: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve job listing contacts: {}", e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn update_job_listing_contact_service(
    pool: &SqlitePool,
    id: &i64,
    job_listing_id: Option<i64>,
    person_id: Option<i64>,
    role: Option<&Role>,
) -> JsonResult {
    info!("Updating job listing contact ID: {}", id);

    let result =
        job_listing_contact::update_job_listing_contact(pool, *id, job_listing_id, person_id, role)
            .await;

    match result {
        Ok(record) => {
            info!("Job listing contact updated. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Job listing contact {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating job listing contact: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to update job listing contact {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn delete_job_listing_contact_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting job listing contact ID: {}", id);

    let result = job_listing_contact::delete_job_listing_contact(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Job listing contact deleted. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Job listing contact {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting job listing contact: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete job listing contact {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
