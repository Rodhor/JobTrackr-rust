use crate::db::models::enums::ContactType;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct Contact {
    pub id: i64,
    pub contact_type: ContactType,
    pub contact_date: String,
    pub location: Option<String>,
    pub user_id: i64,
    pub person_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_contact(
    pool: &SqlitePool,
    contact_type: &ContactType,
    contact_date: &str,
    location: Option<&str>,
    user_id: i64,
    person_id: Option<i64>,
) -> Result<Contact, Error> {
    let contact_type = contact_type.as_str();

    let contact = query_as!(
        Contact,
        r#"
        INSERT INTO contact (
            contact_type,
            contact_date,
            location,
            user_id,
            person_id
        )
        VALUES (?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            user_id,
            person_id,
            created_at,
            updated_at
        "#,
        contact_type,
        contact_date,
        location,
        user_id,
        person_id
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

pub async fn get_contact_by_id(pool: &SqlitePool, id: i64) -> Result<Contact, Error> {
    let contact = query_as!(
        Contact,
        r#"
        SELECT
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            user_id,
            person_id,
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

pub async fn get_all_contacts(pool: &SqlitePool) -> Result<Vec<Contact>, Error> {
    let contacts = query_as!(
        Contact,
        r#"
        SELECT
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            user_id,
            person_id,
            created_at,
            updated_at
        FROM contact
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

pub async fn get_contacts_by_user_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<Contact>, Error> {
    let contacts = query_as!(
        Contact,
        r#"
        SELECT
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            user_id,
            person_id,
            created_at,
            updated_at
        FROM contact
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

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
            user_id,
            person_id,
            created_at,
            updated_at
        FROM contact
        WHERE person_id = ?
        "#,
        person_id
    )
    .fetch_all(pool)
    .await?;

    Ok(contacts)
}

pub async fn update_contact(
    pool: &SqlitePool,
    id: i64,
    contact_type: &ContactType,
    contact_date: &str,
    location: Option<&str>,
    user_id: i64,
    person_id: Option<i64>,
) -> Result<Contact, Error> {
    let contact_type = contact_type.as_str();

    let contact = query_as!(
        Contact,
        r#"
        UPDATE contact
        SET
            contact_type = ?,
            contact_date = ?,
            location = ?,
            user_id = ?,
            person_id = ?
        WHERE id = ?
        RETURNING
            id AS "id!: i64",
            contact_type AS "contact_type: ContactType",
            contact_date,
            location,
            user_id,
            person_id,
            created_at,
            updated_at
        "#,
        contact_type,
        contact_date,
        location,
        user_id,
        person_id,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

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
