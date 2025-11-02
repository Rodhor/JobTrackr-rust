use crate::db::models::enums::NoteType;
use crate::services::note_service::{
    create_note_service, delete_note_service, get_all_notes_service, get_note_by_id_service,
    update_note_service,
};
use crate::services::service_types::JsonResult;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum NoteCommand {
    Create {
        interaction_id: Option<i64>,
        job_listing_id: Option<i64>,
        application_id: Option<i64>,
        person_id: Option<i64>,
        company_id: Option<i64>,
        note_type: Option<NoteType>,
        title: Option<String>,
        content: Option<String>,
    },
    Update {
        id: i64,
        interaction_id: Option<i64>,
        job_listing_id: Option<i64>,
        application_id: Option<i64>,
        person_id: Option<i64>,
        company_id: Option<i64>,
        note_type: Option<NoteType>,
        title: Option<String>,
        content: Option<String>,
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
pub async fn handle_note_command(
    pool: tauri::State<'_, SqlitePool>,
    command: NoteCommand,
) -> JsonResult {
    match command {
        // ======================================================
        // Create
        // ======================================================
        NoteCommand::Create {
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type,
            title,
            content,
        } => {
            create_note_service(
                &pool,
                interaction_id,
                job_listing_id,
                application_id,
                person_id,
                company_id,
                note_type.as_ref(),
                title.as_deref(),
                content.as_deref(),
            )
            .await
        }

        // ======================================================
        // Update
        // ======================================================
        NoteCommand::Update {
            id,
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type,
            title,
            content,
        } => {
            update_note_service(
                &pool,
                &id,
                interaction_id,
                job_listing_id,
                application_id,
                person_id,
                company_id,
                note_type.as_ref(),
                title.as_deref(),
                content.as_deref(),
            )
            .await
        }

        // ======================================================
        // Get by ID
        // ======================================================
        NoteCommand::GetById { id } => get_note_by_id_service(&pool, &id).await,

        // ======================================================
        // List All
        // ======================================================
        NoteCommand::ListAll => get_all_notes_service(&pool).await,

        // ======================================================
        // Delete
        // ======================================================
        NoteCommand::Delete { id } => delete_note_service(&pool, &id).await,
    }
}
