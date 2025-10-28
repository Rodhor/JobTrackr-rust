use crate::db::models::enums::Stage;
use crate::db::queries::application;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use chrono::NaiveDate;
use sqlx::SqlitePool;

// ======================================================
// Create Application
// ======================================================
pub async fn create_application_service(
    pool: &SqlitePool,
    job_listing_id: Option<i64>,
    stage: Option<&Stage>,
    applied_date: &NaiveDate,
    application_notes: Option<&str>,
) -> JsonResult {
    info!(
        "Creating application for job_listing_id: {:?}",
        job_listing_id
    );

    let result = application::create_application(
        pool,
        job_listing_id,
        stage,
        applied_date,
        application_notes,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Application created successfully. ID: {}", record.id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!(
                    "Application created successfully (job_listing_id: {:?}).",
                    job_listing_id
                ),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error creating application: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!(
                    "Failed to create application (job_listing_id: {:?}): {}",
                    job_listing_id, e
                )
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Application by ID
// ======================================================
pub async fn get_application_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving application by ID: {}", id);

    let result = application::get_application_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Application {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error retrieving application: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve application {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Applications
// ======================================================
pub async fn get_all_applications_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all applications");

    let result = application::get_all_applications(pool).await;

    match result {
        Ok(records) => {
            info!("Applications retrieved successfully.");
            let json = serde_json::json!({
                "status": "success",
                "message": "All applications retrieved successfully.",
                "data": records
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error retrieving applications: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Error retrieving applications: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Application
// ======================================================
pub async fn update_application_service(
    pool: &SqlitePool,
    id: &i64,
    job_listing_id: Option<i64>,
    stage: Option<&Stage>,
    applied_date: Option<&NaiveDate>,
    application_notes: Option<&str>,
) -> JsonResult {
    info!("Updating application with ID: {}", id);

    let result = application::update_application(
        pool,
        *id,
        job_listing_id,
        stage,
        applied_date,
        application_notes,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Application updated successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Application {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error updating application: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to update application {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Application
// ======================================================
pub async fn delete_application_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting application with ID: {}", id);

    let result = application::delete_application(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Application deleted successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Application {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error deleting application: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete application {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
