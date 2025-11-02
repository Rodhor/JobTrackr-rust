use crate::db::models::enums::Role;
use crate::services::person_service::{
    create_person_service, delete_person_service, get_all_persons_service,
    get_person_by_id_service, update_person_service,
};
use crate::services::service_types::JsonResult;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum PersonCommand {
    Create {
        first_name: String,
        last_name: String,
        email: Option<String>,
        phone_number: Option<String>,
        role: Option<Role>,
        linkedin_url: Option<String>,
        company_id: Option<i64>,
    },
    Update {
        id: i64,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
        role: Option<Role>,
        linkedin_url: Option<String>,
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
pub async fn handle_person_command(
    pool: tauri::State<'_, SqlitePool>,
    command: PersonCommand,
) -> JsonResult {
    match command {
        PersonCommand::Create {
            first_name,
            last_name,
            email,
            phone_number,
            role,
            linkedin_url,
            company_id,
        } => {
            create_person_service(
                &pool,
                &first_name,
                &last_name,
                email.as_deref(),
                phone_number.as_deref(),
                role.as_ref(),
                linkedin_url.as_deref(),
                company_id,
            )
            .await
        }

        PersonCommand::Update {
            id,
            first_name,
            last_name,
            email,
            phone_number,
            role,
            linkedin_url,
            company_id,
        } => {
            update_person_service(
                &pool,
                &id,
                first_name.as_deref(),
                last_name.as_deref(),
                email.as_deref(),
                phone_number.as_deref(),
                role.as_ref(),
                linkedin_url.as_deref(),
                company_id,
            )
            .await
        }

        PersonCommand::GetById { id } => get_person_by_id_service(&pool, &id).await,
        PersonCommand::ListAll => get_all_persons_service(&pool).await,
        PersonCommand::Delete { id } => delete_person_service(&pool, &id).await,
    }
}
