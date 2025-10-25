use crate::db::queries::user;
use crate::logger::*;
use crate::services::service_types::JsonResult;
use sqlx::SqlitePool;

pub async fn create_user_service(
    pool: &SqlitePool,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
    phone_number: Option<&str>,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
) -> JsonResult {
    info!("Creating user: {:?} {:?}", first_name, last_name);

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
        Ok(record) => {
            info!("User created successfully. ID: {}", record.id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("User '{:?}' '{:?}' created successfully.", first_name, last_name),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error creating user: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to create user '{:?}' '{:?}': {}", first_name, last_name, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_user_by_id_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Retrieving user by ID: {}", id);

    let result = user::get_user_by_id(pool, *id).await;

    match result {
        Ok(record) => {
            let json = serde_json::json!({
                "status": "success",
                "message": format!("User with ID {} retrieved successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error retrieving user: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to retrieve user with ID {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn get_all_users_service(pool: &SqlitePool) -> JsonResult {
    info!("Retrieving all users");
    let result = user::get_all_users(pool).await;
    match result {
        Ok(records) => {
            info!("All users were successfully retrieved.");
            let json = serde_json::json!({
                "status":"success",
                "message": "All users were retrieved successfully",
                "data":records
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Error retrieving all users: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Error retrieving users: {}",e)
            });
            Err(json.to_string())
        }
    }
}

pub async fn update_user_service(
    pool: &SqlitePool,
    id: &i64,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
    phone_number: Option<&str>,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
) -> JsonResult {
    info!("Updating user with ID: {}", id);

    let result = user::update_user(
        pool,
        *id,
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
        Ok(record) => {
            info!("User updated succesfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("User {} updated successfully.", id),
                "data": record
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error updating user: {}", e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Failed to update user with ID {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
pub async fn delete_user_service(pool: &SqlitePool, id: &i64) -> JsonResult {
    info!("Deleting user by ID: {}", id);

    let result = user::delete_user(pool, *id).await;

    match result {
        Ok(_) => {
            info!("User deleted successfully. ID: {}", id);
            let json = serde_json::json!({
                "status": "success",
                "message": format!("User with ID {} deleted successfully.", id)
            });
            Ok(json.to_string())
        }
        Err(e) => {
            error!("Database error deleting user with ID {}: {}", id, e);
            let json = serde_json::json!({
                "status": "error",
                "message": format!("Error deleting user with ID {}: {}", id, e)
            });
            Err(json.to_string())
        }
    }
}
