use crate::db::models::enums::InteractionType;
use crate::db::queries::interaction;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use crate::services::service_utils::add_display_label;
use chrono::NaiveDate;
use serde_json::{json, Value};
use sqlx::SqlitePool;

// ======================================================
// Create Interaction
// ======================================================
pub async fn create_interaction_service(
    pool: &SqlitePool,
    interaction_type: &InteractionType,
    interaction_date: &NaiveDate,
    subject: Option<&str>,
    summary: Option<&str>,
    medium: Option<&str>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
) -> JsonResult {
    info!(
        "Creating interaction (application_id={:?}, person_id={:?}, company_id={:?})",
        application_id, person_id, company_id
    );

    let result = interaction::create_interaction(
        pool,
        interaction_type,
        interaction_date,
        subject,
        summary,
        medium,
        application_id,
        person_id,
        company_id,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Interaction created successfully. ID: {}", record.id);

            let data = add_display_label(&record, record.summary.as_deref());

            let json = json!({
                "status": "success",
                "message": "Interaction created successfully.",
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error creating interaction: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to create interaction: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get Interaction by ID
// ======================================================
pub async fn get_interaction_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving interaction by ID: {}", id);

    let result = interaction::get_interaction_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            info!("Interaction retrieved successfully. ID: {}", id);

            let data = add_display_label(&record, record.summary.as_deref());

            let json = json!({
                "status": "success",
                "message": format!("Interaction {} retrieved successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving interaction {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve interaction {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Get All Interactions
// ======================================================
pub async fn get_all_interactions_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all interactions");

    let result = interaction::get_all_interactions(pool).await;

    match result {
        Ok(records) => {
            info!(
                "Interactions retrieved successfully ({} total).",
                records.len()
            );

            let data: Vec<Value> = records
                .into_iter()
                .map(|r| add_display_label(&r, r.summary.as_deref()))
                .collect();

            let json = json!({
                "status": "success",
                "message": "All interactions retrieved successfully.",
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving interactions: {}", e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to retrieve interactions: {}", e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Update Interaction
// ======================================================
pub async fn update_interaction_service(
    pool: &SqlitePool,
    id: &i64,
    interaction_type: Option<&InteractionType>,
    interaction_date: Option<&NaiveDate>,
    subject: Option<&str>,
    summary: Option<&str>,
    medium: Option<&str>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
) -> JsonResult {
    info!("Updating interaction ID: {}", id);

    let result = interaction::update_interaction(
        pool,
        *id,
        interaction_type,
        interaction_date,
        subject,
        summary,
        medium,
        application_id,
        person_id,
        company_id,
    )
    .await;

    match result {
        Ok(record) => {
            info!("Interaction updated successfully. ID: {}", id);

            let data = add_display_label(&record, record.summary.as_deref());

            let json = json!({
                "status": "success",
                "message": format!("Interaction {} updated successfully.", id),
                "data": data
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error updating interaction {}: {}", id, e);
            let json = json!({
                "status": "error",
                "message": format!("Failed to update interaction {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

// ======================================================
// Delete Interaction
// ======================================================
pub async fn delete_interaction_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting interaction ID: {}", id);

    let result = interaction::delete_interaction(pool, *id).await;

    match result {
        Ok(_) => {
            info!("Interaction deleted successfully. ID: {}", id);

            let json = json!({
                "status": "success",
                "message": format!("Interaction {} deleted successfully.", id)
            });

            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error deleting interaction {}: {}", id, e);

            let json = json!({
                "status": "error",
                "message": format!("Failed to delete interaction {}: {}", id, e)
            });

            Err(json.to_string())
        }
    }
}
