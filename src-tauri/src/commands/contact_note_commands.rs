use crate::db::models::enums::NoteType;
use crate::services::contact_note_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_contact_note_command(
    pool: tauri::State<'_, SqlitePool>,
    contact_id: i64,
    note_type: Option<NoteType>,
    content: Option<String>,
) -> JsonResult {
    contact_note_service::create_contact_note_service(
        &pool,
        contact_id,
        note_type.as_ref(),
        content.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn get_contact_note_by_id_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
) -> JsonResult {
    contact_note_service::get_contact_note_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_contact_notes_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    contact_note_service::get_all_contact_notes_service(&pool).await
}

#[tauri::command]
pub async fn update_contact_note_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    contact_id: Option<i64>,
    note_type: Option<NoteType>,
    content: Option<String>,
) -> JsonResult {
    contact_note_service::update_contact_note_service(
        &pool,
        &id,
        contact_id,
        note_type.as_ref(),
        content.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn delete_contact_note_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
) -> JsonResult {
    contact_note_service::delete_contact_note_service(&pool, &id).await
}
