use crate::db::user;
use crate::logger; // your existing logger module
use serde_json::json;
use sqlx::SqlitePool;

pub async fn create_user_service(
    pool: &SqlitePool,
    first_name: &str,
    last_name: &str,
    email: &str,
    phone_number: Option<&str>,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
) -> Result<String, String> {
    // log start
    logger::info(&format!("Creating user: {} {}", first_name, last_name));

    // execute DB call
    let result = user::create_user(
        pool,
        first_name,
        last_name,
        email,
        phone_number,
        street_address,
        zip_code,
        city,
        country,
    )
    .await;

    match result {
        Ok(u) => {
            let json =
                serde_json::to_string(&u).map_err(|e| format!("Serialization error: {}", e))?;
            logger::info(&format!("User created: id={}", u.id));
            Ok(json)
        }
        Err(e) => {
            logger::error(&format!("DB error creating user: {}", e));
            Err(format!("Database error: {}", e))
        }
    }
}
