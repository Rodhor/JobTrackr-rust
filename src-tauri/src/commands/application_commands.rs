use crate::db::models::enums::Status;
use crate::services::application_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_application_command(
    pool: tauri::State<'_, SqlitePool>,
    user_id: i64,
    job_listing_id: i64,
    status: Option<Status>,
    applied_date: String,
    cv_file_path: Option<String>,
    cover_letter_file_path: Option<String>,
    application_notes: Option<String>,
) -> JsonResult {
    application_service::create_application_service(
        &pool,
        user_id,
        job_listing_id,
        status.as_ref(),
        &applied_date,
        cv_file_path.as_deref(),
        cover_letter_file_path.as_deref(),
        application_notes.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn get_application_by_id_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
) -> JsonResult {
    application_service::get_application_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_applications_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    application_service::get_all_applications_service(&pool).await
}

#[tauri::command]
pub async fn update_application_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    user_id: Option<i64>,
    job_listing_id: Option<i64>,
    status: Option<Status>,
    applied_date: Option<String>,
    cv_file_path: Option<String>,
    cover_letter_file_path: Option<String>,
    application_notes: Option<String>,
) -> JsonResult {
    application_service::update_application_service(
        &pool,
        &id,
        user_id,
        job_listing_id,
        status.as_ref(),
        applied_date.as_deref(),
        cv_file_path.as_deref(),
        cover_letter_file_path.as_deref(),
        application_notes.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn delete_application_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    application_service::delete_application_service(&pool, &id).await
}
