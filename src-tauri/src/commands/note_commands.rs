use crate::db::models::enums::NoteType;
use crate::services::note_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

// ======================================================
// Create Note Command
// ======================================================
#[tauri::command]
pub async fn create_note_command(
    pool: tauri::State<'_, SqlitePool>,
    contact_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<NoteType>,
    content: Option<String>,
) -> JsonResult {
    note_service::create_note_service(
        &pool,
        contact_id,
        job_listing_id,
        application_id,
        person_id,
        company_id,
        note_type.as_ref(),
        content.as_deref(),
    )
    .await
}

// ======================================================
// Get Note by ID Command
// ======================================================
#[tauri::command]
pub async fn get_note_by_id_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    note_service::get_note_by_id_service(&pool, &id).await
}

// ======================================================
// Get All Notes Command
// ======================================================
#[tauri::command]
pub async fn get_all_notes_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    note_service::get_all_notes_service(&pool).await
}

// ======================================================
// Update Note Command
// ======================================================
#[tauri::command]
pub async fn update_note_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    contact_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<NoteType>,
    content: Option<String>,
) -> JsonResult {
    note_service::update_note_service(
        &pool,
        &id,
        contact_id,
        job_listing_id,
        application_id,
        person_id,
        company_id,
        note_type.as_ref(),
        content.as_deref(),
    )
    .await
}

// ======================================================
// Delete Note Command
// ======================================================
#[tauri::command]
pub async fn delete_note_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    note_service::delete_note_service(&pool, &id).await
}
