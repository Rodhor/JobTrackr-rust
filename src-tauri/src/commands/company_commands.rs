use crate::db::models::enums::WorkType;
use crate::services::company_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_company_command(
    pool: tauri::State<'_, SqlitePool>,
    name: String,
    street_address: Option<String>,
    zip_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
    default_work_type: Option<WorkType>,
    industry: Option<String>,
    website: Option<String>,
    phone_number: Option<String>,
) -> JsonResult {
    company_service::create_company_service(
        &pool,
        &name,
        street_address.as_deref(),
        zip_code.as_deref(),
        city.as_deref(),
        country.as_deref(),
        default_work_type.as_ref(),
        industry.as_deref(),
        website.as_deref(),
        phone_number.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn get_company_by_id_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    company_service::get_company_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_companies_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    company_service::get_all_companies_service(&pool).await
}

#[tauri::command]
pub async fn update_company_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    name: Option<String>,
    street_address: Option<String>,
    zip_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
    default_work_type: Option<WorkType>,
    industry: Option<String>,
    website: Option<String>,
    phone_number: Option<String>,
) -> JsonResult {
    company_service::update_company_service(
        &pool,
        &id,
        name.as_deref(),
        street_address.as_deref(),
        zip_code.as_deref(),
        city.as_deref(),
        country.as_deref(),
        default_work_type.as_ref(),
        industry.as_deref(),
        website.as_deref(),
        phone_number.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn delete_company_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    company_service::delete_company_service(&pool, &id).await
}
