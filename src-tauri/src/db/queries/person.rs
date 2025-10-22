use crate::db::models::enums::Role;
use crate::utils::sql_utils::build_update_sql;
use chrono::Utc;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Person {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<Role>,
    pub linkedin_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_person(
    pool: &SqlitePool,
    first_name: &str,
    last_name: &str,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
) -> Result<Person, Error> {
    let role_str = role.map(|r| r.as_str());
    let now = Utc::now().to_rfc3339();

    let person = query_as!(
        Person,
        r#"
        INSERT INTO person (
            first_name,
            last_name,
            email,
            phone_number,
            role,
            linkedin_url,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            first_name,
            last_name,
            email,
            phone_number,
            role AS "role: Role",
            linkedin_url,
            created_at,
            updated_at
        "#,
        first_name,
        last_name,
        email,
        phone_number,
        role_str,
        linkedin_url,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(person)
}

pub async fn get_person_by_id(pool: &SqlitePool, id: i64) -> Result<Person, Error> {
    let person = query_as!(
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
            created_at,
            updated_at
        FROM person
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(person)
}

pub async fn get_all_persons(pool: &SqlitePool) -> Result<Vec<Person>, Error> {
    let persons = query_as!(
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
            created_at,
            updated_at
        FROM person
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(persons)
}

pub async fn update_person(
    pool: &SqlitePool,
    id: i64,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
) -> Result<Person, Error> {
    // Map role enum to &str if provided
    let role_str = role.map(|r| r.as_str());

    // 1. Build SQL dynamically
    let fields = vec![
        ("first_name", first_name),
        ("last_name", last_name),
        ("email", email),
        ("phone_number", phone_number),
        ("role", role_str),
        ("linkedin_url", linkedin_url),
    ];

    let (sql, binds) = build_update_sql("person", "id", id, fields);

    // 2. Bind parameters in the right order
    let mut query = sqlx::query_as::<_, Person>(&sql);
    for val in binds.iter().take(binds.len() - 1) {
        query = query.bind(val);
    }
    // Last param is the id
    query = query.bind(binds.last().unwrap());

    // 3. Execute and return updated record
    let person = query.fetch_one(pool).await?;
    Ok(person)
}

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
