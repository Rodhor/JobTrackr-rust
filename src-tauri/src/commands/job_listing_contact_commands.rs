use crate::db::models::enums::Role;
use crate::services::job_listing_contact_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_job_listing_contact_command(
    pool: tauri::State<'_, SqlitePool>,
    job_listing_id: i64,
    person_id: i64,
    role: Option<Role>,
) -> JsonResult {
    job_listing_contact_service::create_job_listing_contact_service(
        &pool,
        job_listing_id,
        person_id,
        role.as_ref(),
    )
    .await
}

#[tauri::command]
pub async fn get_job_listing_contact_by_id_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
) -> JsonResult {
    job_listing_contact_service::get_job_listing_contact_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_job_listing_contacts_command(
    pool: tauri::State<'_, SqlitePool>,
) -> JsonResult {
    job_listing_contact_service::get_all_job_listing_contacts_service(&pool).await
}

#[tauri::command]
pub async fn update_job_listing_contact_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    job_listing_id: Option<i64>,
    person_id: Option<i64>,
    role: Option<Role>,
) -> JsonResult {
    job_listing_contact_service::update_job_listing_contact_service(
        &pool,
        &id,
        job_listing_id,
        person_id,
        role.as_ref(),
    )
    .await
}

#[tauri::command]
pub async fn delete_job_listing_contact_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
) -> JsonResult {
    job_listing_contact_service::delete_job_listing_contact_service(&pool, &id).await
}
