use crate::db::models::enums::Stage;
use crate::services::application_service;
use crate::services::service_types::JsonResult;
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_application_command(
    pool: tauri::State<'_, SqlitePool>,
    job_listing_id: Option<i64>,
    stage: Option<Stage>,
    applied_date: String,
    application_notes: Option<String>,
) -> JsonResult {
    let parsed_date = match NaiveDate::parse_from_str(&applied_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            let json = serde_json::json!({
                "status": "error",
                "message": "Invalid date format. Expected YYYY-MM-DD."
            });
            return Err(json.to_string());
        }
    };

    application_service::create_application_service(
        &pool,
        job_listing_id,
        stage.as_ref(),
        &parsed_date,
        application_notes.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn update_application_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    job_listing_id: Option<i64>,
    stage: Option<Stage>,
    applied_date: Option<String>,
    application_notes: Option<String>,
) -> JsonResult {
    let parsed_date = match applied_date {
        Some(ref d) => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            Ok(v) => Some(v),
            Err(_) => {
                let json = serde_json::json!({
                    "status": "error",
                    "message": "Invalid date format. Expected YYYY-MM-DD."
                });
                return Err(json.to_string());
            }
        },
        None => None,
    };

    application_service::update_application_service(
        &pool,
        &id,
        job_listing_id,
        stage.as_ref(),
        parsed_date.as_ref(),
        application_notes.as_deref(),
    )
    .await
}

// ======================================================
// Get Application by ID Command
// ======================================================
#[tauri::command]
pub async fn get_application_by_id_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
) -> JsonResult {
    application_service::get_application_by_id_service(&pool, &id).await
}

// ======================================================
// Get All applications Command
// ======================================================
#[tauri::command]
pub async fn get_all_applications_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    application_service::get_all_applications_service(&pool).await
}
