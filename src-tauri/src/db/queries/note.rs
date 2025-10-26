use crate::db::models::enums::NoteType;
use crate::utils::sql_utils::build_update_sql;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Note {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub job_listing_id: Option<i64>,
    pub application_id: Option<i64>,
    pub person_id: Option<i64>,
    pub company_id: Option<i64>,
    pub note_type: Option<NoteType>,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_note(
    pool: &SqlitePool,
    contact_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<&NoteType>,
    content: Option<&str>,
) -> Result<Note, Error> {
    let note_type_str = note_type.map(|t| t.as_str());

    let note = query_as!(
        Note,
        r#"
        INSERT INTO note (
            contact_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type,
            content
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            contact_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        "#,
        contact_id,
        job_listing_id,
        application_id,
        person_id,
        company_id,
        note_type_str,
        content
    )
    .fetch_one(pool)
    .await?;

    Ok(note)
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_note_by_id(pool: &SqlitePool, id: i64) -> Result<Note, Error> {
    let note = query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            contact_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        FROM note
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(note)
}

// ======================================================
// Get all
// ======================================================
pub async fn get_all_notes(pool: &SqlitePool) -> Result<Vec<Note>, Error> {
    let notes = query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            contact_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        FROM note
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

// ======================================================
// Filtered Retrievals
// ======================================================
pub async fn get_notes_by_application_id(
    pool: &SqlitePool,
    application_id: i64,
) -> Result<Vec<Note>, Error> {
    let notes = query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            contact_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        FROM note
        WHERE application_id = ?
        ORDER BY created_at DESC
        "#,
        application_id
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

pub async fn get_notes_by_contact_id(
    pool: &SqlitePool,
    contact_id: i64,
) -> Result<Vec<Note>, Error> {
    let notes = query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            contact_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        FROM note
        WHERE contact_id = ?
        ORDER BY created_at DESC
        "#,
        contact_id
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

// ======================================================
// Update
// ======================================================
pub async fn update_note(
    pool: &SqlitePool,
    id: i64,
    contact_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<&NoteType>,
    content: Option<&str>,
) -> Result<Note, Error> {
    let note_type_str = note_type.map(|t| t.as_str());
    let contact_id_s = contact_id.map(|v| v.to_string());
    let job_listing_id_s = job_listing_id.map(|v| v.to_string());
    let application_id_s = application_id.map(|v| v.to_string());
    let person_id_s = person_id.map(|v| v.to_string());
    let company_id_s = company_id.map(|v| v.to_string());

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("contact_id", contact_id_s.as_deref()),
        ("job_listing_id", job_listing_id_s.as_deref()),
        ("application_id", application_id_s.as_deref()),
        ("person_id", person_id_s.as_deref()),
        ("company_id", company_id_s.as_deref()),
        ("note_type", note_type_str),
        ("content", content),
    ];

    let (sql, binds) = build_update_sql("note", "id", id, fields);

    let mut query = sqlx::query_as::<_, Note>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    let note = query.fetch_one(pool).await?;
    Ok(note)
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_note(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM note
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
