use crate::db::models::enums::Role;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
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
    let role = role.map(|r| r.as_str());

    let person = query_as!(
        Person,
        r#"
        INSERT INTO person (
            first_name,
            last_name,
            email,
            phone_number,
            role,
            linkedin_url
        )
        VALUES (?, ?, ?, ?, ?, ?)
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
        role,
        linkedin_url
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
    first_name: &str,
    last_name: &str,
    email: Option<&str>,
    phone_number: Option<&str>,
    role: Option<&Role>,
    linkedin_url: Option<&str>,
) -> Result<Person, Error> {
    let role = role.map(|r| r.as_str());

    let person = query_as!(
        Person,
        r#"
        UPDATE person
        SET
            first_name = ?,
            last_name = ?,
            email = ?,
            phone_number = ?,
            role = ?,
            linkedin_url = ?
        WHERE id = ?
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
        role,
        linkedin_url,
        id
    )
    .fetch_one(pool)
    .await?;

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
