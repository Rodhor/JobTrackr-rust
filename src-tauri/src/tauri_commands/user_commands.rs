use crate::services::user_service;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn create_user_command(
    pool: tauri::State<'_, SqlitePool>,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: Option<String>,
    street_address: Option<String>,
    zip_code: Option<String>,
    city: Option<String>,
    country: Option<String>,
) -> Result<String, String> {
    user_service::create_user_service(
        &pool,
        &first_name,
        &last_name,
        &email,
        phone_number.as_deref(),
        street_address.as_deref(),
        zip_code.as_deref(),
        city.as_deref(),
        country.as_deref(),
    )
    .await
}
