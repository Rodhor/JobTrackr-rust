use crate::utils::sql_utils::build_update_sql;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Reminder {
    pub id: i64,
    pub application_id: Option<i64>,
    pub interaction_id: Option<i64>,
    pub note_id: Option<i64>,
    pub job_listing_id: Option<i64>,
    pub company_id: Option<i64>,
    pub person_id: Option<i64>,
    pub reminder_date: NaiveDate,
    pub title: String,
    pub message: Option<String>,
    pub is_completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_reminder(
    pool: &SqlitePool,
    application_id: Option<i64>,
    interaction_id: Option<i64>,
    note_id: Option<i64>,
    job_listing_id: Option<i64>,
    company_id: Option<i64>,
    person_id: Option<i64>,
    reminder_date: &NaiveDate,
    title: &str,
    message: Option<&str>,
    is_completed: bool,
) -> Result<Reminder, Error> {
    let reminder_date_str = reminder_date.format("%Y-%m-%d").to_string();

    let reminder = query_as!(
        Reminder,
        r#"
        INSERT INTO reminder (
            application_id,
            interaction_id,
            note_id,
            job_listing_id,
            company_id,
            person_id,
            reminder_date,
            title,
            message,
            is_completed
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            application_id,
            interaction_id,
            note_id,
            job_listing_id,
            company_id,
            person_id,
            reminder_date AS "reminder_date!: NaiveDate",
            title AS "title!: String",
            message,
            is_completed as "is_completed!: bool",
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        "#,
        application_id,
        interaction_id,
        note_id,
        job_listing_id,
        company_id,
        person_id,
        reminder_date_str,
        title,
        message,
        is_completed
    )
    .fetch_one(pool)
    .await?;

    Ok(reminder)
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_reminder_by_id(pool: &SqlitePool, id: i64) -> Result<Reminder, Error> {
    query_as!(
        Reminder,
        r#"
        SELECT
            id AS "id!: i64",
            application_id,
            interaction_id,
            note_id,
            job_listing_id,
            company_id,
            person_id,
            reminder_date AS "reminder_date!: NaiveDate",
            title AS "title!: String",
            message,
            is_completed as "is_completed!: bool",
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM reminder
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
pub async fn get_all_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, Error> {
    query_as!(
        Reminder,
        r#"
        SELECT
            id AS "id!: i64",
            application_id,
            interaction_id,
            note_id,
            job_listing_id,
            company_id,
            person_id,
            reminder_date AS "reminder_date!: NaiveDate",
            title AS "title!: String",
            message,
            is_completed as "is_completed!: bool",
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM reminder
        ORDER BY reminder_date ASC
        "#
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Get upcoming (uncompleted)
// ======================================================
pub async fn get_upcoming_reminders(
    pool: &SqlitePool,
    current_date: &NaiveDate,
) -> Result<Vec<Reminder>, Error> {
    let current_date_str = current_date.format("%Y-%m-%d").to_string();

    query_as!(
        Reminder,
        r#"
        SELECT
            id AS "id!: i64",
            application_id,
            interaction_id,
            note_id,
            job_listing_id,
            company_id,
            person_id,
            reminder_date AS "reminder_date!: NaiveDate",
            title AS "title!: String",
            message,
            is_completed as "is_completed!: bool",
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM reminder
        WHERE reminder_date >= ? AND is_completed = 0
        ORDER BY reminder_date ASC
        "#,
        current_date_str
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Update
// ======================================================
pub async fn update_reminder(
    pool: &SqlitePool,
    id: i64,
    application_id: Option<i64>,
    interaction_id: Option<i64>,
    note_id: Option<i64>,
    job_listing_id: Option<i64>,
    company_id: Option<i64>,
    person_id: Option<i64>,
    reminder_date: Option<&NaiveDate>,
    title: Option<&str>,
    message: Option<&str>,
    is_completed: Option<bool>,
) -> Result<Reminder, Error> {
    let application_id_s = application_id.map(|v| v.to_string());
    let interaction_id_s = interaction_id.map(|v| v.to_string());
    let note_id_s = note_id.map(|v| v.to_string());
    let job_listing_id_s = job_listing_id.map(|v| v.to_string());
    let company_id_s = company_id.map(|v| v.to_string());
    let person_id_s = person_id.map(|v| v.to_string());
    let is_completed_s = is_completed.map(|v| if v { "1".to_string() } else { "0".to_string() });
    let reminder_date_s = reminder_date.map(|d| d.format("%Y-%m-%d").to_string());
    let reminder_date_ref = reminder_date_s.as_deref();

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("application_id", application_id_s.as_deref()),
        ("interaction_id", interaction_id_s.as_deref()),
        ("note_id", note_id_s.as_deref()),
        ("job_listing_id", job_listing_id_s.as_deref()),
        ("company_id", company_id_s.as_deref()),
        ("person_id", person_id_s.as_deref()),
        ("reminder_date", reminder_date_ref),
        ("title", title),
        ("message", message),
        ("is_completed", is_completed_s.as_deref()),
    ];

    let (sql, binds) = build_update_sql("reminder", "id", id, fields);

    let mut query = sqlx::query_as::<_, Reminder>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    query.fetch_one(pool).await
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_reminder(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM reminder
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
