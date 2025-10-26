use crate::db::models::enums::ContactType;
use crate::utils::sql_utils::build_update_sql;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Contact {
    pub id: i64,
    pub contact_type: ContactType,
    pub contact_date: NaiveDate,
    pub location: Option<String>,
    pub person_id: Option<i64>,
    pub application_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_contact(
    pool: &SqlitePool,
    contact_type: &ContactType,
    contact_date: &NaiveDate,
    location: Option<&str>,
    person_id: Option<i64>,
    application_id: Option<i64>,
) -> Result<Contact, Error> {
    let contact_type_str = contact_type.as_str();

    let contact = query_as!(
        Contact,
        r#"
        INSERT INTO contact (
            contact_type,
            contact_date,
            location,
            person_id,
            application_id
        )
        VALUES (?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            person_id,
            application_id,
            created_at,
            updated_at
        "#,
        contact_type_str,
        contact_date, // <-- directly pass NaiveDate
        location,
        person_id,
        application_id
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_contact_by_id(pool: &SqlitePool, id: i64) -> Result<Contact, Error> {
    let contact = query_as!(
        Contact,
        r#"
        SELECT
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            person_id,
            application_id,
            created_at,
            updated_at
        FROM contact
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

// ======================================================
// Get all
// ======================================================
pub async fn get_all_contacts(pool: &SqlitePool) -> Result<Vec<Contact>, Error> {
    let contacts = query_as!(
        Contact,
        r#"
        SELECT
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            person_id,
            application_id,
            created_at,
            updated_at
        FROM contact
        ORDER BY contact_date DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

// ======================================================
// Get by Person ID
// ======================================================
pub async fn get_contacts_by_person_id(
    pool: &SqlitePool,
    person_id: i64,
) -> Result<Vec<Contact>, Error> {
    let contacts = query_as!(
        Contact,
        r#"
        SELECT
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            person_id,
            application_id,
            created_at,
            updated_at
        FROM contact
        WHERE person_id = ?
        ORDER BY contact_date DESC
        "#,
        person_id
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

// ======================================================
// Get by Application ID
// ======================================================
pub async fn get_contacts_by_application_id(
    pool: &SqlitePool,
    application_id: i64,
) -> Result<Vec<Contact>, Error> {
    let contacts = query_as!(
        Contact,
        r#"
        SELECT
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            person_id,
            application_id,
            created_at,
            updated_at
        FROM contact
        WHERE application_id = ?
        ORDER BY contact_date DESC
        "#,
        application_id
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

// ======================================================
// Update
// ======================================================
pub async fn update_contact(
    pool: &SqlitePool,
    id: i64,
    contact_type: Option<&ContactType>,
    contact_date: Option<&NaiveDate>,
    location: Option<&str>,
    person_id: Option<i64>,
    application_id: Option<i64>,
) -> Result<Contact, Error> {
    let contact_type_str = contact_type.map(|c| c.as_str());
    let person_id_s = person_id.map(|v| v.to_string());
    let application_id_s = application_id.map(|v| v.to_string());

    // convert NaiveDate to formatted ISO string (for SQL text binding)
    let contact_date_s = contact_date.map(|d| d.format("%Y-%m-%d").to_string());
    let contact_date_ref = contact_date_s.as_deref();

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("contact_type", contact_type_str),
        ("contact_date", contact_date_ref),
        ("location", location),
        ("person_id", person_id_s.as_deref()),
        ("application_id", application_id_s.as_deref()),
    ];

    let (sql, binds) = build_update_sql("contact", "id", id, fields);

    let mut query = sqlx::query_as::<_, Contact>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    let contact = query.fetch_one(pool).await?;
    Ok(contact)
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_contact(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM contact
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
