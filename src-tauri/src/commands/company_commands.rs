use crate::db::models::enums::WorkType;
use crate::services::company_service::{
    create_company_service, delete_company_service, get_all_companies_service,
    get_company_by_id_service, update_company_service,
};
use crate::services::service_types::JsonResult;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum CompanyCommand {
    #[serde(rename_all = "camelCase")]
    Create {
        name: String,
        street_address: Option<String>,
        zip_code: Option<String>,
        city: Option<String>,
        country: Option<String>,
        default_work_type: Option<WorkType>,
        industry: Option<String>,
        website: Option<String>,
        phone_number: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Update {
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
pub async fn handle_company_command(
    pool: tauri::State<'_, SqlitePool>,
    command: CompanyCommand,
) -> JsonResult {
    match command {
        // ======================================================
        // Create
        // ======================================================
        CompanyCommand::Create {
            name,
            street_address,
            zip_code,
            city,
            country,
            default_work_type,
            industry,
            website,
            phone_number,
        } => {
            create_company_service(
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

        // ======================================================
        // Update
        // ======================================================
        CompanyCommand::Update {
            id,
            name,
            street_address,
            zip_code,
            city,
            country,
            default_work_type,
            industry,
            website,
            phone_number,
        } => {
            update_company_service(
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

        // ======================================================
        // Get by ID
        // ======================================================
        CompanyCommand::GetById { id } => get_company_by_id_service(&pool, &id).await,

        // ======================================================
        // List All
        // ======================================================
        CompanyCommand::ListAll => get_all_companies_service(&pool).await,

        // ======================================================
        // Delete
        // ======================================================
        CompanyCommand::Delete { id } => delete_company_service(&pool, &id).await,
    }
}
