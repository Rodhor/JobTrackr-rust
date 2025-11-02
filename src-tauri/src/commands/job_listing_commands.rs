use crate::db::models::enums::{Currency, SeniorityLevel, WorkType};
use crate::services::job_listing_service::{
    create_job_listing_service, delete_job_listing_service, get_all_job_listings_service,
    get_job_listing_by_id_service, update_job_listing_service,
};
use crate::services::service_types::JsonResult;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum JobListingCommand {
    Create {
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
    },
    Update {
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
    },
    GetById {
        id: i64,
    },
    ListAll,
    Delete {
        id: i64,
    },
}

#[tauri::command]
pub async fn handle_job_listing_command(
    pool: tauri::State<'_, SqlitePool>,
    command: JobListingCommand,
) -> JsonResult {
    match command {
        // ======================================================
        // Create
        // ======================================================
        JobListingCommand::Create {
            company_id,
            title,
            work_type,
            category,
            seniority_level,
            salary_min,
            salary_max,
            currency,
            description,
            url,
        } => {
            create_job_listing_service(
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

        // ======================================================
        // Update
        // ======================================================
        JobListingCommand::Update {
            id,
            company_id,
            title,
            work_type,
            category,
            seniority_level,
            salary_min,
            salary_max,
            currency,
            description,
            url,
        } => {
            update_job_listing_service(
                &pool,
                &id,
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

        // ======================================================
        // Get by ID
        // ======================================================
        JobListingCommand::GetById { id } => get_job_listing_by_id_service(&pool, &id).await,

        // ======================================================
        // List All
        // ======================================================
        JobListingCommand::ListAll => get_all_job_listings_service(&pool).await,

        // ======================================================
        // Delete
        // ======================================================
        JobListingCommand::Delete { id } => delete_job_listing_service(&pool, &id).await,
    }
}
