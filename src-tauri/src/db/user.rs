use chrono::Utc;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
    pub zip_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_user(
    pool: &SqlitePool,
    first_name: &str,
    last_name: &str,
    email: &str,
    phone_number: Option<&str>,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
) -> Result<User, Error> {
    let now = Utc::now().to_rfc3339();

    let user = query_as!(
        User,
        r#"
        INSERT INTO user (
            first_name,
            last_name,
            email,
            phone_number,
            street_address,
            zip_code,
            city,
            country,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            first_name,
            last_name,
            email,
            phone_number,
            street_address,
            zip_code,
            city,
            country,
            created_at,
            updated_at
        "#,
        first_name,
        last_name,
        email,
        phone_number,
        street_address,
        zip_code,
        city,
        country,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &SqlitePool, id: &i64) -> Result<User, Error> {
    let user = query_as!(
        User,
        r#"
        SELECT
            id,
            first_name,
            last_name,
            email,
            phone_number,
            street_address,
            zip_code,
            city,
            country,
            created_at,
            updated_at
        FROM user
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, Error> {
    let users = query_as!(
        User,
        r#"
        SELECT
            id,
            first_name,
            last_name,
            email,
            phone_number,
            street_address,
            zip_code,
            city,
            country,
            created_at,
            updated_at
        FROM user
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn update_user(
    pool: &SqlitePool,
    id: &i64,
    first_name: &str,
    last_name: &str,
    email: &str,
    phone_number: Option<&str>,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
) -> Result<User, Error> {
    let now = Utc::now().to_rfc3339();

    let user = query_as!(
        User,
        r#"
        UPDATE user
        SET
            first_name = ?,
            last_name = ?,
            email = ?,
            phone_number = ?,
            street_address = ?,
            zip_code = ?,
            city = ?,
            country = ?,
            updated_at = ?
        WHERE id = ?
        RETURNING
            id AS "id!: i64",
            first_name,
            last_name,
            email,
            phone_number,
            street_address,
            zip_code,
            city,
            country,
            created_at,
            updated_at
        "#,
        first_name,
        last_name,
        email,
        phone_number,
        street_address,
        zip_code,
        city,
        country,
        now,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &SqlitePool, id: &i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM user
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
