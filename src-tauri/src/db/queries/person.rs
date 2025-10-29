use crate::db::models::enums::Role;
use crate::utils::sql_utils::build_update_sql;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<Role>,
    pub linkedin_url: Option<String>,
    pub company_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_person(
    pool: &SqlitePool,
    first_name: &str,
    last_name: &str,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
    company_id: Option<i64>,
) -> Result<Person, Error> {
    let role_str = role.map(|r| r.as_str());

    query_as!(
        Person,
        r#"
        INSERT INTO person (
            first_name,
            last_name,
            email,
            phone_number,
            role,
            linkedin_url,
            company_id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            first_name,
            last_name,
            email,
            phone_number,
            role AS "role: Role",
            linkedin_url,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        "#,
        first_name,
        last_name,
        email,
        phone_number,
        role_str,
        linkedin_url,
        company_id
    )
    .fetch_one(pool)
    .await
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_person_by_id(pool: &SqlitePool, id: i64) -> Result<Person, Error> {
    query_as!(
        Person,
        r#"
        SELECT
            id AS "id!: i64",
            first_name,
            last_name,
            email,
            phone_number,
            role AS "role: Role",
            linkedin_url,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM person
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

// ======================================================
// Get all
// ======================================================
pub async fn get_all_persons(pool: &SqlitePool) -> Result<Vec<Person>, Error> {
    query_as!(
        Person,
        r#"
        SELECT
            id AS "id!: i64",
            first_name,
            last_name,
            email,
            phone_number,
            role AS "role: Role",
            linkedin_url,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM person
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Get all by Company ID
// ======================================================
pub async fn get_persons_by_company_id(
    pool: &SqlitePool,
    company_id: i64,
) -> Result<Vec<Person>, Error> {
    query_as!(
        Person,
        r#"
        SELECT
            id AS "id!: i64",
            first_name,
            last_name,
            email,
            phone_number,
            role AS "role: Role",
            linkedin_url,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM person
        WHERE company_id = ?
        ORDER BY created_at DESC
        "#,
        company_id
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Update
// ======================================================
pub async fn update_person(
    pool: &SqlitePool,
    id: i64,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
    company_id: Option<i64>,
) -> Result<Person, Error> {
    let role_str = role.map(|r| r.as_str());
    let company_id_s = company_id.map(|v| v.to_string());

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("first_name", first_name),
        ("last_name", last_name),
        ("email", email),
        ("phone_number", phone_number),
        ("role", role_str),
        ("linkedin_url", linkedin_url),
        ("company_id", company_id_s.as_deref()),
    ];

    let (sql, binds) = build_update_sql("person", "id", id, fields);

    let mut query = sqlx::query_as::<_, Person>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    query.fetch_one(pool).await
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_person(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM person
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
