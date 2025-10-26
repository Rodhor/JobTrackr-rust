use crate::db::models::enums::ContactType;
use crate::services::contact_service;
use crate::services::service_types::JsonResult;
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_contact_command(
    pool: tauri::State<'_, SqlitePool>,
    contact_type: ContactType,
    contact_date: String,
    location: Option<String>,
    person_id: Option<i64>,
    application_id: Option<i64>,
) -> JsonResult {
    let parsed_date = match NaiveDate::parse_from_str(&contact_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            let json = serde_json::json!({
                "status": "error",
                "message": "Invalid date format. Expected YYYY-MM-DD."
            });
            return Err(json.to_string());
        }
    };

    contact_service::create_contact_service(
        &pool,
        &contact_type,
        &parsed_date,
        location.as_deref(),
        person_id,
        application_id,
    )
    .await
}

#[tauri::command]
pub async fn update_contact_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    contact_type: Option<ContactType>,
    contact_date: Option<String>,
    location: Option<String>,
    person_id: Option<i64>,
    application_id: Option<i64>,
) -> JsonResult {
    let parsed_date = match contact_date {
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

    contact_service::update_contact_service(
        &pool,
        &id,
        contact_type.as_ref(),
        parsed_date.as_ref(),
        location.as_deref(),
        person_id,
        application_id,
    )
    .await
}
