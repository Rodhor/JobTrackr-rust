use crate::db::models::enums::WorkType;
use crate::db::queries::company;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use crate::services::service_utils::add_display_label;
use serde_json::{json, Value};
use sqlx::SqlitePool;

// ======================================================
// Create Company
// ======================================================
pub async fn create_company_service(
    pool: &SqlitePool,
    name: &str,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
    default_work_type: Option<&WorkType>,
    industry: Option<&str>,
    website: Option<&str>,
    phone_number: Option<&str>,
) -> JsonResult {
    info!("Creating company: {}", name);

    let result = company::create_company(
        pool,
        name,
        street_address,
        zip_code,
        city,
        country,
        default_work_type,
        industry,
        website,
        phone_number,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Company created successfully. ID: {}", record.id);

            let data = add_display_label(&record, Some(record.name.as_str()));

            let json = json!({
                "status": "success",
                "message": format!("Company '{}' created successfully.", record.name),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating company '{}': {}", name, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to create company '{}': {}", name, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Company by ID
// ======================================================
pub async fn get_company_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving company by ID: {}", id);

    let result = company::get_company_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            info!("Company retrieved successfully. ID: {}", id);

            let data = add_display_label(&record, Some(record.name.as_str()));

            let json = json!({
                "status": "success",
                "message": format!("Company {} retrieved successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving company {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve company {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Companies
// ======================================================
pub async fn get_all_companies_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all companies");

    let result = company::get_all_companies(pool).await;

    match result {
        Ok(records) => {
            info!(
                "Companies retrieved successfully ({} total).",
                records.len()
            );

            let data: Vec<Value> = records
                .into_iter()
                .map(|r| add_display_label(&r, Some(r.name.as_str())))
                .collect();

            let json = json!({
                "status": "success",
                "message": "All companies retrieved successfully.",
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving companies: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve companies: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Company
// ======================================================
pub async fn update_company_service(
    pool: &SqlitePool,
    id: &i64,
    name: Option<&str>,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
    default_work_type: Option<&WorkType>,
    industry: Option<&str>,
    website: Option<&str>,
    phone_number: Option<&str>,
) -> JsonResult {
    info!("Updating company with ID: {}", id);

    let result = company::update_company(
        pool,
        *id,
        name,
        street_address,
        zip_code,
        city,
        country,
        default_work_type,
        industry,
        website,
        phone_number,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Company updated successfully. ID: {}", id);

            let data = add_display_label(&record, Some(record.name.as_str()));

            let json = json!({
                "status": "success",
                "message": format!("Company {} updated successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating company {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to update company {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Company
// ======================================================
pub async fn delete_company_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting company with ID: {}", id);

    let result = company::delete_company(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Company deleted successfully. ID: {}", id);

            let json = json!({
                "status": "success",
                "message": format!("Company {} deleted successfully.", id)
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting company {}: {}", id, e);

            let json = json!({
                "status": "error",
                "message": format!("Failed to delete company {}: {}", id, e)
            });

            Err(json.to_string())
        }
    }
}
