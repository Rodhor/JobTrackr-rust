use crate::db::models::enums::Role;
use crate::services::person_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_person_command(
    pool: tauri::State<'_, SqlitePool>,
    first_name: String,
    last_name: String,
    email: Option<String>,
    phone_number: Option<String>,
    role: Option<Role>,
    linkedin_url: Option<String>,
) -> JsonResult {
    person_service::create_person_service(
        &pool,
        &first_name,
        &last_name,
        email.as_deref(),
        phone_number.as_deref(),
        role.as_ref(),
        linkedin_url.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn get_person_by_id_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    person_service::get_person_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_persons_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    person_service::get_all_persons_service(&pool).await
}

#[tauri::command]
pub async fn update_person_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    role: Option<Role>,
    linkedin_url: Option<String>,
) -> JsonResult {
    person_service::update_person_service(
        &pool,
        &id,
        first_name.as_deref(),
        last_name.as_deref(),
        email.as_deref(),
        phone_number.as_deref(),
        role.as_ref(),
        linkedin_url.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn delete_person_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    person_service::delete_person_service(&pool, &id).await
}
