use crate::db::models::enums::NoteType;
use crate::utils::sql_utils::build_update_sql;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Note {
    pub id: i64,
    pub interaction_id: Option<i64>,
    pub job_listing_id: Option<i64>,
    pub application_id: Option<i64>,
    pub person_id: Option<i64>,
    pub company_id: Option<i64>,
    pub note_type: Option<NoteType>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_note(
    pool: &SqlitePool,
    interaction_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<&NoteType>,
    title: Option<&str>,
    content: Option<&str>,
) -> Result<Note, sqlx::Error> {
    sqlx::query_as!(
        Note,
        r#"
        INSERT INTO note (
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type,
            title,
            content
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            title,
            content,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        "#,
        interaction_id,
        job_listing_id,
        application_id,
        person_id,
        company_id,
        note_type,
        title,
        content
    )
    .fetch_one(pool)
    .await
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_note_by_id(pool: &SqlitePool, id: i64) -> Result<Note, Error> {
    query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            title,
            content,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM note
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
pub async fn get_all_notes(pool: &SqlitePool) -> Result<Vec<Note>, Error> {
    query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            title,
            content,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM note
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Filtered Retrievals
// ======================================================
pub async fn get_notes_by_application_id(
    pool: &SqlitePool,
    application_id: i64,
) -> Result<Vec<Note>, Error> {
    query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            title,
            content,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM note
        WHERE application_id = ?
        ORDER BY created_at DESC
        "#,
        application_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_notes_by_interaction_id(
    pool: &SqlitePool,
    interaction_id: i64,
) -> Result<Vec<Note>, Error> {
    query_as!(
        Note,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_id,
            job_listing_id,
            application_id,
            person_id,
            company_id,
            note_type AS "note_type: NoteType",
            title,
            content,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM note
        WHERE interaction_id = ?
        ORDER BY created_at DESC
        "#,
        interaction_id
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Update
// ======================================================
pub async fn update_note(
    pool: &SqlitePool,
    id: i64,
    interaction_id: Option<i64>,
    job_listing_id: Option<i64>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
    note_type: Option<&NoteType>,
    title: Option<&str>,
    content: Option<&str>,
) -> Result<Note, Error> {
    let note_type_str = note_type.map(|t| t.as_str());
    let interaction_id_s = interaction_id.map(|v| v.to_string());
    let job_listing_id_s = job_listing_id.map(|v| v.to_string());
    let application_id_s = application_id.map(|v| v.to_string());
    let person_id_s = person_id.map(|v| v.to_string());
    let company_id_s = company_id.map(|v| v.to_string());

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("interaction_id", interaction_id_s.as_deref()),
        ("job_listing_id", job_listing_id_s.as_deref()),
        ("application_id", application_id_s.as_deref()),
        ("person_id", person_id_s.as_deref()),
        ("company_id", company_id_s.as_deref()),
        ("note_type", note_type_str),
        ("title", title),
        ("content", content),
    ];

    let (sql, binds) = build_update_sql("note", "id", id, fields);

    let mut query = sqlx::query_as::<_, Note>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    query.fetch_one(pool).await
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
