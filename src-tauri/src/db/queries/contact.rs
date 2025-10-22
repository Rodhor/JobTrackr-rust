use crate::db::models::enums::ContactType;
use crate::utils::sql_utils::build_update_sql;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
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
    let contact_type_str = contact_type.as_str();

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
        contact_type_str,
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
    contact_type: Option<&ContactType>,
    contact_date: Option<&str>,
    location: Option<&str>,
    user_id: Option<i64>,
    person_id: Option<i64>,
) -> Result<Contact, Error> {
    let contact_type_str = contact_type.map(|c| c.as_str());
    let user_id_s = user_id.map(|v| v.to_string());
    let person_id_s = person_id.map(|v| v.to_string());

    // Build SQL dynamically
    let fields = vec![
        ("contact_type", contact_type_str),
        ("contact_date", contact_date),
        ("location", location),
        ("user_id", user_id_s.as_deref()),
        ("person_id", person_id_s.as_deref()),
    ];

    let (sql, binds) = build_update_sql("contact", "id", id, fields);

    // Bind parameters
    let mut query = sqlx::query_as::<_, Contact>(&sql);
    for val in binds.iter().take(binds.len() - 1) {
        query = query.bind(val);
    }
    query = query.bind(binds.last().unwrap());

    // Execute and return updated record
    let contact = query.fetch_one(pool).await?;
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
