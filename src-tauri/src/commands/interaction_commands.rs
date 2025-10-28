use crate::db::models::enums::InteractionType;
use crate::services::interaction_service;
use crate::services::service_types::JsonResult;
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_interaction_command(
    pool: tauri::State<'_, SqlitePool>,
    interaction_type: InteractionType,
    interaction_date: String,
    subject: Option<String>,
    summary: Option<String>,
    medium: Option<String>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
) -> JsonResult {
    let parsed_date = match NaiveDate::parse_from_str(&interaction_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            let json = serde_json::json!({
                "status": "error",
                "message": "Invalid date format. Expected YYYY-MM-DD."
            });
            return Err(json.to_string());
        }
    };

    interaction_service::create_interaction_service(
        &pool,
        &interaction_type,
        &parsed_date,
        subject.as_deref(),
        summary.as_deref(),
        medium.as_deref(),
        application_id,
        person_id,
        company_id,
    )
    .await
}

#[tauri::command]
pub async fn update_interaction_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    interaction_type: Option<InteractionType>,
    interaction_date: Option<String>,
    subject: Option<String>,
    summary: Option<String>,
    medium: Option<String>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
) -> JsonResult {
    let parsed_date = match interaction_date {
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

    interaction_service::update_interaction_service(
        &pool,
        &id,
        interaction_type.as_ref(),
        parsed_date.as_ref(),
        subject.as_deref(),
        summary.as_deref(),
        medium.as_deref(),
        application_id,
        person_id,
        company_id,
    )
    .await
}
