use crate::services::service_types::JsonResult;
use crate::services::user_service;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_user_command(
    pool: tauri::State<'_, SqlitePool>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    street_address: Option<String>,
    zip_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
) -> JsonResult {
    user_service::create_user_service(
        &pool,
        first_name.as_deref(),
        last_name.as_deref(),
        email.as_deref(),
        phone_number.as_deref(),
        street_address.as_deref(),
        zip_code.as_deref(),
        city.as_deref(),
        country.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn get_user_by_id_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    user_service::get_user_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_users_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    user_service::get_all_users_service(&pool).await
}

#[tauri::command]
pub async fn update_user_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    street_address: Option<String>,
    zip_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
) -> JsonResult {
    user_service::update_user_service(
        &pool,
        &id,
        first_name.as_deref(),
        last_name.as_deref(),
        email.as_deref(),
        phone_number.as_deref(),
        street_address.as_deref(),
        zip_code.as_deref(),
        city.as_deref(),
        country.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn delete_user_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    user_service::delete_user_service(&pool, &id).await
}
