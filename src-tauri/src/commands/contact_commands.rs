use crate::db::models::enums::ContactType;
use crate::services::contact_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_contact_command(
    pool: tauri::State<'_, SqlitePool>,
    contact_type: ContactType,
    contact_date: String,
    location: Option<String>,
    user_id: i64,
    person_id: Option<i64>,
) -> JsonResult {
    contact_service::create_contact_service(
        &pool,
        &contact_type,
        &contact_date,
        location.as_deref(),
        user_id,
        person_id,
    )
    .await
}

#[tauri::command]
pub async fn get_contact_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    contact_service::get_contact_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_contacts_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    contact_service::get_all_contacts_service(&pool).await
}

#[tauri::command]
pub async fn update_contact_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    contact_type: Option<ContactType>,
    contact_date: Option<String>,
    location: Option<String>,
    user_id: Option<i64>,
    person_id: Option<i64>,
) -> JsonResult {
    contact_service::update_contact_service(
        &pool,
        &id,
        contact_type.as_ref(),
        contact_date.as_deref(),
        location.as_deref(),
        user_id,
        person_id,
    )
    .await
}

#[tauri::command]
pub async fn delete_contact_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    contact_service::delete_contact_service(&pool, &id).await
}
