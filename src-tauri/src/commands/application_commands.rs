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
    cv_file_path: Option<String>,
    cover_letter_file_path: Option<String>,
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
        cv_file_path.as_deref(),
        cover_letter_file_path.as_deref(),
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
    cv_file_path: Option<String>,
    cover_letter_file_path: Option<String>,
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
        cv_file_path.as_deref(),
        cover_letter_file_path.as_deref(),
        application_notes.as_deref(),
    )
    .await
}
