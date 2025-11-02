use crate::command_utils::{parse_optional_date, parse_required_date};
use crate::db::models::enums::InteractionType;
use crate::services::interaction_service::{
    create_interaction_service, delete_interaction_service, get_all_interactions_service,
    get_interaction_by_id_service, update_interaction_service,
};
use crate::services::service_types::JsonResult;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum InteractionCommand {
    Create {
        interaction_type: InteractionType,
        interaction_date: String,
        subject: Option<String>,
        summary: Option<String>,
        medium: Option<String>,
        application_id: Option<i64>,
        person_id: Option<i64>,
        company_id: Option<i64>,
    },
    Update {
        id: i64,
        interaction_type: Option<InteractionType>,
        interaction_date: Option<String>,
        subject: Option<String>,
        summary: Option<String>,
        medium: Option<String>,
        application_id: Option<i64>,
        person_id: Option<i64>,
        company_id: Option<i64>,
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
pub async fn handle_interaction_command(
    pool: tauri::State<'_, SqlitePool>,
    command: InteractionCommand,
) -> JsonResult {
    match command {
        // ======================================================
        // Create
        // ======================================================
        InteractionCommand::Create {
            interaction_type,
            interaction_date,
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
        } => {
            let parsed_date = parse_required_date(interaction_date)?;

            create_interaction_service(
                &pool,
                &interaction_type,
                &parsed_date,
                subject.as_deref(),
                summary.as_deref(),
                medium.as_deref(),
                application_id,
                person_id,
                company_id,
            )
            .await
        }

        // ======================================================
        // Update
        // ======================================================
        InteractionCommand::Update {
            id,
            interaction_type,
            interaction_date,
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
        } => {
            let parsed_date = parse_optional_date(interaction_date)?;

            update_interaction_service(
                &pool,
                &id,
                interaction_type.as_ref(),
                parsed_date.as_ref(),
                subject.as_deref(),
                summary.as_deref(),
                medium.as_deref(),
                application_id,
                person_id,
                company_id,
            )
            .await
        }

        // ======================================================
        // Get by ID
        // ======================================================
        InteractionCommand::GetById { id } => get_interaction_by_id_service(&pool, &id).await,

        // ======================================================
        // List All
        // ======================================================
        InteractionCommand::ListAll => get_all_interactions_service(&pool).await,

        // ======================================================
        // Delete
        // ======================================================
        InteractionCommand::Delete { id } => delete_interaction_service(&pool, &id).await,
    }
}
