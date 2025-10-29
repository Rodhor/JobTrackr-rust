use crate::db::models::enums::Stage;
use crate::utils::sql_utils::build_update_sql;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::{query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub id: i64,
    pub job_listing_id: Option<i64>,
    pub stage: Option<Stage>,
    pub applied_date: NaiveDate,
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
    application_notes: Option<&str>,
) -> Result<Application, Error> {
    let stage_str = stage.map(|s| s.as_str());
    let applied_date_str = applied_date.format("%Y-%m-%d").to_string();

    query_as!(
        Application,
        r#"
        INSERT INTO application (
            job_listing_id,
            stage,
            applied_date,
            application_notes
        )
        VALUES (?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date AS "applied_date!: NaiveDate",
            application_notes,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        "#,
        job_listing_id,
        stage_str,
        applied_date_str,
        application_notes
    )
    .fetch_one(pool)
    .await
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_application_by_id(pool: &SqlitePool, id: i64) -> Result<Application, Error> {
    query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date AS "applied_date!: NaiveDate",
            application_notes,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM application
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
pub async fn get_all_applications(pool: &SqlitePool) -> Result<Vec<Application>, Error> {
    query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date AS "applied_date!: NaiveDate",
            application_notes,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM application
        ORDER BY applied_date DESC
        "#
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Get by Job Listing ID
// ======================================================
pub async fn get_applications_by_job_listing_id(
    pool: &SqlitePool,
    job_listing_id: i64,
) -> Result<Vec<Application>, Error> {
    query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            job_listing_id,
            stage AS "stage: Stage",
            applied_date AS "applied_date!: NaiveDate",
            application_notes,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM application
        WHERE job_listing_id = ?
        ORDER BY applied_date DESC
        "#,
        job_listing_id
    )
    .fetch_all(pool)
    .await
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
    application_notes: Option<&str>,
) -> Result<Application, Error> {
    let job_listing_id_str = job_listing_id.map(|v| v.to_string());
    let stage_str = stage.map(|s| s.as_str());
    let applied_date_str = applied_date.map(|d| d.format("%Y-%m-%d").to_string());
    let applied_date_ref = applied_date_str.as_deref();

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("job_listing_id", job_listing_id_str.as_deref()),
        ("stage", stage_str),
        ("applied_date", applied_date_ref),
        ("application_notes", application_notes),
    ];

    let (sql, binds) = build_update_sql("application", "id", id, fields);

    let mut query = sqlx::query_as::<_, Application>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    query.fetch_one(pool).await
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_application(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = sqlx::query!(
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
