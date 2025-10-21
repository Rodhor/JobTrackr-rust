use crate::db::enums::NoteType;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct ContactNote {
    pub id: i64,
    pub contact_id: i64,
    pub note_type: Option<NoteType>,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_contact_note(
    pool: &SqlitePool,
    contact_id: i64,
    note_type: Option<&NoteType>,
    content: Option<&str>,
) -> Result<ContactNote, Error> {
    let note_type = note_type.map(|n| n.as_str());

    let note = query_as!(
        ContactNote,
        r#"
        INSERT INTO contact_note (
            contact_id,
            note_type,
            content
        )
        VALUES (?, ?, ?)
        RETURNING
            id AS "id!: i64",
            contact_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        "#,
        contact_id,
        note_type,
        content
    )
    .fetch_one(pool)
    .await?;

    Ok(note)
}

pub async fn get_contact_note_by_id(pool: &SqlitePool, id: i64) -> Result<ContactNote, Error> {
    let note = query_as!(
        ContactNote,
        r#"
        SELECT
            id AS "id!: i64",
            contact_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        FROM contact_note
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(note)
}

pub async fn get_all_contact_notes(pool: &SqlitePool) -> Result<Vec<ContactNote>, Error> {
    let notes = query_as!(
        ContactNote,
        r#"
        SELECT
            id AS "id!: i64",
            contact_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        FROM contact_note
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

pub async fn get_contact_notes_by_contact_id(
    pool: &SqlitePool,
    contact_id: i64,
) -> Result<Vec<ContactNote>, Error> {
    let notes = query_as!(
        ContactNote,
        r#"
        SELECT
            id AS "id!: i64",
            contact_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        FROM contact_note
        WHERE contact_id = ?
        "#,
        contact_id
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

pub async fn update_contact_note(
    pool: &SqlitePool,
    id: i64,
    contact_id: i64,
    note_type: Option<&NoteType>,
    content: Option<&str>,
) -> Result<ContactNote, Error> {
    let note_type = note_type.map(|n| n.as_str());

    let note = query_as!(
        ContactNote,
        r#"
        UPDATE contact_note
        SET
            contact_id = ?,
            note_type = ?,
            content = ?
        WHERE id = ?
        RETURNING
            id AS "id!: i64",
            contact_id,
            note_type AS "note_type: NoteType",
            content,
            created_at,
            updated_at
        "#,
        contact_id,
        note_type,
        content,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(note)
}

pub async fn delete_contact_note(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM contact_note
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
