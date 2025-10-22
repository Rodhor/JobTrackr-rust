use crate::db::models::enums::Role;
use crate::utils::sql_utils::build_update_sql;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
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
    let role_str = role.map(|r| r.as_str());

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
        role_str
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
    job_listing_id: Option<i64>,
    person_id: Option<i64>,
    role: Option<&Role>,
) -> Result<JobListingContact, sqlx::Error> {
    let role_str = role.map(|r| r.as_str());

    // keep owned Strings alive for the lifetime of this function
    let job_listing_id_s = job_listing_id.map(|v| v.to_string());
    let person_id_s = person_id.map(|v| v.to_string());

    // 1) Build SQL with stable &str refs
    let fields = vec![
        ("job_listing_id", job_listing_id_s.as_deref()),
        ("person_id", person_id_s.as_deref()),
        ("role", role_str),
    ];

    let (sql, binds) = build_update_sql("job_listing_contact", "id", id, fields);

    // 2) Bind in order (all but last are SET/updated_at, last is id)
    let mut q = sqlx::query_as::<_, JobListingContact>(&sql);
    for v in binds.iter().take(binds.len() - 1) {
        q = q.bind(v);
    }
    q = q.bind(binds.last().unwrap());

    // 3) Execute
    let contact = q.fetch_one(pool).await?;
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
