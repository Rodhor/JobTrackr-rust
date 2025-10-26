use crate::db::models::enums::Stage;
use crate::utils::sql_utils::build_update_sql;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Application {
    pub id: i64,
    pub job_listing_id: Option<i64>,
    pub stage: Option<Stage>,
    pub applied_date: NaiveDate,
    pub cv_file_path: Option<String>,
    pub cover_letter_file_path: Option<String>,
    pub application_notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_application(
    pool: &SqlitePool,
    job_listing_id: Option<i64>,
    stage: Option<&Stage>,
    applied_date: &NaiveDate,
    cv_file_path: Option<&str>,
    cover_letter_file_path: Option<&str>,
    application_notes: Option<&str>,
) -> Result<Application, Error> {
    let stage_str = stage.map(|s| s.as_str());

    let application = query_as!(
        Application,
        r#"
        INSERT INTO application (
            job_listing_id,
            stage,
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes
        )
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        "#,
        job_listing_id,
        stage_str,
        applied_date, // <-- NaiveDate passed directly
        cv_file_path,
        cover_letter_file_path,
        application_notes
    )
    .fetch_one(pool)
    .await?;

    Ok(application)
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_application_by_id(pool: &SqlitePool, id: i64) -> Result<Application, Error> {
    let application = query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        FROM application
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(application)
}

// ======================================================
// Get all
// ======================================================
pub async fn get_all_applications(pool: &SqlitePool) -> Result<Vec<Application>, Error> {
    let applications = query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        FROM application
        ORDER BY applied_date DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(applications)
}

// ======================================================
// Get by Job Listing ID
// ======================================================
pub async fn get_applications_by_job_listing_id(
    pool: &SqlitePool,
    job_listing_id: i64,
) -> Result<Vec<Application>, Error> {
    let applications = query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        FROM application
        WHERE job_listing_id = ?
        ORDER BY applied_date DESC
        "#,
        job_listing_id
    )
    .fetch_all(pool)
    .await?;

    Ok(applications)
}

// ======================================================
// Update
// ======================================================
pub async fn update_application(
    pool: &SqlitePool,
    id: i64,
    job_listing_id: Option<i64>,
    stage: Option<&Stage>,
    applied_date: Option<&NaiveDate>,
    cv_file_path: Option<&str>,
    cover_letter_file_path: Option<&str>,
    application_notes: Option<&str>,
) -> Result<Application, Error> {
    let job_listing_id_s = job_listing_id.map(|v| v.to_string());
    let stage_str = stage.map(|s| s.as_str());

    // build_update_sql expects homogeneous Option<&str> fields
    // but we can keep dates as formatted ISO strings
    let applied_date_str = applied_date.map(|d| d.format("%Y-%m-%d").to_string());
    let applied_date_ref = applied_date_str.as_deref();

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("job_listing_id", job_listing_id_s.as_deref()),
        ("stage", stage_str),
        ("applied_date", applied_date_ref),
        ("cv_file_path", cv_file_path),
        ("cover_letter_file_path", cover_letter_file_path),
        ("application_notes", application_notes),
    ];

    let (sql, binds) = build_update_sql("application", "id", id, fields);

    let mut query = sqlx::query_as::<_, Application>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    let application = query.fetch_one(pool).await?;
    Ok(application)
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_application(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM application
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
