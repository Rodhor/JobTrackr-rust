use crate::db::models::enums::{Currency, SeniorityLevel, WorkType};
use crate::db::queries::job_listing;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

// ======================================================
// Create Job Listing
// ======================================================
pub async fn create_job_listing_service(
    pool: &SqlitePool,
    company_id: i64,
    title: &str,
    work_type: Option<&WorkType>,
    category: Option<&str>,
    seniority_level: Option<&SeniorityLevel>,
    salary_min: Option<i64>,
    salary_max: Option<i64>,
    currency: Option<&Currency>,
    description: Option<&str>,
    url: Option<&str>,
) -> JsonResult {
    info!("Creating job listing '{}'", title);

    let result = job_listing::create_job_listing(
        pool,
        company_id,
        title,
        work_type,
        category,
        seniority_level,
        salary_min,
        salary_max,
        currency,
        description,
        url,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Job listing created successfully. ID: {}", record.id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Job listing '{}' created successfully.", title),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating job listing '{}': {}", title, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to create job listing '{}': {}", title, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Job Listing by ID
// ======================================================
pub async fn get_job_listing_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving job listing ID: {}", id);

    let result = job_listing::get_job_listing_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            info!("Job listing retrieved successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Job listing {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving job listing {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve job listing {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Job Listings
// ======================================================
pub async fn get_all_job_listings_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all job listings");

    let result = job_listing::get_all_job_listings(pool).await;

    match result {
        Ok(records) => {
            info!(
                "Job listings retrieved successfully ({} total).",
                records.len()
            );
            let json = serde_json::json!({
                "status": "success",
                "message": "All job listings retrieved successfully.",
                "data": records
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving job listings: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve job listings: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Job Listing
// ======================================================
pub async fn update_job_listing_service(
    pool: &SqlitePool,
    id: &i64,
    company_id: Option<i64>,
    title: Option<&str>,
    work_type: Option<&WorkType>,
    category: Option<&str>,
    seniority_level: Option<&SeniorityLevel>,
    salary_min: Option<i64>,
    salary_max: Option<i64>,
    currency: Option<&Currency>,
    description: Option<&str>,
    url: Option<&str>,
) -> JsonResult {
    info!("Updating job listing ID: {}", id);

    let result = job_listing::update_job_listing(
        pool,
        *id,
        company_id,
        title,
        work_type,
        category,
        seniority_level,
        salary_min,
        salary_max,
        currency,
        description,
        url,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Job listing updated successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Job listing {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating job listing {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to update job listing {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Job Listing
// ======================================================
pub async fn delete_job_listing_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting job listing ID: {}", id);

    let result = job_listing::delete_job_listing(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Job listing deleted successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("Job listing {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting job listing {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to delete job listing {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
