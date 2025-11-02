use crate::commands::command_utils::{parse_optional_date, parse_required_date};
use crate::db::models::enums::Stage;
use crate::services::application_service::{
    create_application_service, delete_application_service, get_all_applications_service,
    get_application_by_id_service, update_application_service,
};
use crate::services::service_types::JsonResult;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ApplicationCommand {
    Create {
        job_listing_id: Option<i64>,
        stage: Option<Stage>,
        applied_date: String,
        application_notes: Option<String>,
    },
    Update {
        id: i64,
        job_listing_id: Option<i64>,
        stage: Option<Stage>,
        applied_date: Option<String>,
        application_notes: Option<String>,
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
pub async fn handle_application_command(
    pool: tauri::State<'_, SqlitePool>,
    command: ApplicationCommand,
) -> JsonResult {
    match command {
        // ======================================================
        // Create
        // ======================================================
        ApplicationCommand::Create {
            job_listing_id,
            stage,
            applied_date,
            application_notes,
        } => {
            let parsed_date = parse_required_date(applied_date)?;

            create_application_service(
                &pool,
                job_listing_id,
                stage.as_ref(),
                &parsed_date,
                application_notes.as_deref(),
            )
            .await
        }

        // ======================================================
        // Update
        // ======================================================
        ApplicationCommand::Update {
            id,
            job_listing_id,
            stage,
            applied_date,
            application_notes,
        } => {
            let parsed_date = parse_optional_date(applied_date)?;

            update_application_service(
                &pool,
                &id,
                job_listing_id,
                stage.as_ref(),
                parsed_date.as_ref(),
                application_notes.as_deref(),
            )
            .await
        }

        // ======================================================
        // Get by ID
        // ======================================================
        ApplicationCommand::GetById { id } => get_application_by_id_service(&pool, &id).await,

        // ======================================================
        // List All
        // ======================================================
        ApplicationCommand::ListAll => get_all_applications_service(&pool).await,

        // ======================================================
        // Delete
        // ======================================================
        ApplicationCommand::Delete { id } => delete_application_service(&pool, &id).await,
    }
}
