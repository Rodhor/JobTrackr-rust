use crate::db::enums::Role;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct JobListingContact {
    pub id: i64,
    pub job_listing_id: i64,
    pub person_id: i64,
    pub role: Option<Role>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_job_listing_contact(
    pool: &SqlitePool,
    job_listing_id: i64,
    person_id: i64,
    role: Option<&Role>,
) -> Result<JobListingContact, Error> {
    let role = role.map(|r| r.as_str());

    let contact = query_as!(
        JobListingContact,
        r#"
        INSERT INTO job_listing_contact (
            job_listing_id,
            person_id,
            role
        )
        VALUES (?, ?, ?)
        RETURNING
            id AS "id!: i64",
            job_listing_id,
            person_id,
            role AS "role: Role",
            created_at,
            updated_at
        "#,
        job_listing_id,
        person_id,
        role
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

pub async fn get_job_listing_contact_by_id(
    pool: &SqlitePool,
    id: i64,
) -> Result<JobListingContact, Error> {
    let contact = query_as!(
        JobListingContact,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            person_id,
            role AS "role: Role",
            created_at,
            updated_at
        FROM job_listing_contact
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

pub async fn get_all_job_listing_contacts(
    pool: &SqlitePool,
) -> Result<Vec<JobListingContact>, Error> {
    let contacts = query_as!(
        JobListingContact,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            person_id,
            role AS "role: Role",
            created_at,
            updated_at
        FROM job_listing_contact
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

pub async fn get_job_listing_contacts_by_listing_id(
    pool: &SqlitePool,
    job_listing_id: i64,
) -> Result<Vec<JobListingContact>, Error> {
    let contacts = query_as!(
        JobListingContact,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            person_id,
            role AS "role: Role",
            created_at,
            updated_at
        FROM job_listing_contact
        WHERE job_listing_id = ?
        "#,
        job_listing_id
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

pub async fn get_job_listing_contacts_by_person_id(
    pool: &SqlitePool,
    person_id: i64,
) -> Result<Vec<JobListingContact>, Error> {
    let contacts = query_as!(
        JobListingContact,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            person_id,
            role AS "role: Role",
            created_at,
            updated_at
        FROM job_listing_contact
        WHERE person_id = ?
        "#,
        person_id
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

pub async fn update_job_listing_contact(
    pool: &SqlitePool,
    id: i64,
    job_listing_id: i64,
    person_id: i64,
    role: Option<&Role>,
) -> Result<JobListingContact, Error> {
    let role = role.map(|r| r.as_str());

    let contact = query_as!(
        JobListingContact,
        r#"
        UPDATE job_listing_contact
        SET
            job_listing_id = ?,
            person_id = ?,
            role = ?
        WHERE id = ?
        RETURNING
            id AS "id!: i64",
            job_listing_id,
            person_id,
            role AS "role: Role",
            created_at,
            updated_at
        "#,
        job_listing_id,
        person_id,
        role,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

pub async fn delete_job_listing_contact(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM job_listing_contact
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
