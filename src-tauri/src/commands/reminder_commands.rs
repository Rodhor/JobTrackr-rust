use crate::command_utils::{parse_optional_date, parse_required_date};
use crate::services::reminder_service::{
    create_reminder_service, delete_reminder_service, get_all_reminders_service,
    get_reminder_by_id_service, update_reminder_service,
};
use crate::services::service_types::JsonResult;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ReminderCommand {
    #[serde(rename_all = "camelCase")]
    Create {
        application_id: Option<i64>,
        interaction_id: Option<i64>,
        note_id: Option<i64>,
        job_listing_id: Option<i64>,
        company_id: Option<i64>,
        person_id: Option<i64>,
        reminder_date: String,
        title: String,
        message: Option<String>,
        is_completed: bool,
    },
    #[serde(rename_all = "camelCase")]
    Update {
        id: i64,
        application_id: Option<i64>,
        interaction_id: Option<i64>,
        note_id: Option<i64>,
        job_listing_id: Option<i64>,
        company_id: Option<i64>,
        person_id: Option<i64>,
        reminder_date: Option<String>,
        title: Option<String>,
        message: Option<String>,
        is_completed: Option<bool>,
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
pub async fn handle_reminder_command(
    pool: tauri::State<'_, SqlitePool>,
    command: ReminderCommand,
) -> JsonResult {
    match command {
        // ======================================================
        // Create
        // ======================================================
        ReminderCommand::Create {
            application_id,
            interaction_id,
            note_id,
            job_listing_id,
            company_id,
            person_id,
            reminder_date,
            title,
            message,
            is_completed,
        } => {
            let parsed_date = parse_required_date(reminder_date)?;
            create_reminder_service(
                &pool,
                application_id,
                interaction_id,
                note_id,
                job_listing_id,
                company_id,
                person_id,
                &parsed_date,
                &title,
                message.as_deref(),
                is_completed,
            )
            .await
        }

        // ======================================================
        // Update
        // ======================================================
        ReminderCommand::Update {
            id,
            application_id,
            interaction_id,
            note_id,
            job_listing_id,
            company_id,
            person_id,
            reminder_date,
            title,
            message,
            is_completed,
        } => {
            let parsed_date = parse_optional_date(reminder_date)?;
            update_reminder_service(
                &pool,
                &id,
                application_id,
                interaction_id,
                note_id,
                job_listing_id,
                company_id,
                person_id,
                parsed_date.as_ref(),
                title.as_deref(),
                message.as_deref(),
                is_completed,
            )
            .await
        }

        // ======================================================
        // Get by ID
        // ======================================================
        ReminderCommand::GetById { id } => get_reminder_by_id_service(&pool, &id).await,

        // ======================================================
        // List All
        // ======================================================
        ReminderCommand::ListAll => get_all_reminders_service(&pool).await,

        // ======================================================
        // Delete
        // ======================================================
        ReminderCommand::Delete { id } => delete_reminder_service(&pool, &id).await,
    }
}
