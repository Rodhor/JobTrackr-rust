use crate::db::models::enums::{Currency, SeniorityLevel, WorkType};
use crate::services::job_listing_service;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_job_listing_command(
    pool: tauri::State<'_, SqlitePool>,
    company_id: i64,
    title: String,
    work_type: Option<WorkType>,
    category: Option<String>,
    seniority_level: Option<SeniorityLevel>,
    salary_min: Option<i64>,
    salary_max: Option<i64>,
    currency: Option<Currency>,
    description: Option<String>,
    url: Option<String>,
) -> JsonResult {
    job_listing_service::create_job_listing_service(
        &pool,
        company_id,
        &title,
        work_type.as_ref(),
        category.as_deref(),
        seniority_level.as_ref(),
        salary_min,
        salary_max,
        currency.as_ref(),
        description.as_deref(),
        url.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn get_job_listing_by_id_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
) -> JsonResult {
    job_listing_service::get_job_listing_by_id_service(&pool, &id).await
}

#[tauri::command]
pub async fn get_all_job_listings_command(pool: tauri::State<'_, SqlitePool>) -> JsonResult {
    job_listing_service::get_all_job_listings_service(&pool).await
}

#[tauri::command]
pub async fn update_job_listing_command(
    pool: tauri::State<'_, SqlitePool>,
    id: i64,
    company_id: Option<i64>,
    title: Option<String>,
    work_type: Option<WorkType>,
    category: Option<String>,
    seniority_level: Option<SeniorityLevel>,
    salary_min: Option<i64>,
    salary_max: Option<i64>,
    currency: Option<Currency>,
    description: Option<String>,
    url: Option<String>,
) -> JsonResult {
    job_listing_service::update_job_listing_service(
        &pool,
        id,
        company_id,
        title.as_deref(),
        work_type.as_ref(),
        category.as_deref(),
        seniority_level.as_ref(),
        salary_min,
        salary_max,
        currency.as_ref(),
        description.as_deref(),
        url.as_deref(),
    )
    .await
}

#[tauri::command]
pub async fn delete_job_listing_command(pool: tauri::State<'_, SqlitePool>, id: i64) -> JsonResult {
    job_listing_service::delete_job_listing_service(&pool, &id).await
}
