use crate::db::enums::Status;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct Application {
    pub id: i64,
    pub user_id: i64,
    pub job_listing_id: i64,
    pub status: Option<Status>,
    pub applied_date: String,
    pub cv_file_path: Option<String>,
    pub cover_letter_file_path: Option<String>,
    pub application_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_application(
    pool: &SqlitePool,
    user_id: i64,
    job_listing_id: i64,
    status: Option<&Status>,
    applied_date: &str,
    cv_file_path: Option<&str>,
    cover_letter_file_path: Option<&str>,
    application_notes: Option<&str>,
) -> Result<Application, Error> {
    let status = status.map(|s| s.as_str());

    let application = query_as!(
        Application,
        r#"
        INSERT INTO application (
            user_id,
            job_listing_id,
            status,
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            user_id,
            job_listing_id,
            status AS "status: Status",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        "#,
        user_id,
        job_listing_id,
        status,
        applied_date,
        cv_file_path,
        cover_letter_file_path,
        application_notes
    )
    .fetch_one(pool)
    .await?;

    Ok(application)
}

pub async fn get_application_by_id(pool: &SqlitePool, id: i64) -> Result<Application, Error> {
    let application = query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            user_id,
            job_listing_id,
            status AS "status: Status",
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

pub async fn get_all_applications(pool: &SqlitePool) -> Result<Vec<Application>, Error> {
    let applications = query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            user_id,
            job_listing_id,
            status AS "status: Status",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        FROM application
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(applications)
}

pub async fn get_applications_by_user_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<Application>, Error> {
    let applications = query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            user_id,
            job_listing_id,
            status AS "status: Status",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        FROM application
        WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(applications)
}

pub async fn get_applications_by_job_listing_id(
    pool: &SqlitePool,
    job_listing_id: i64,
) -> Result<Vec<Application>, Error> {
    let applications = query_as!(
        Application,
        r#"
        SELECT
            id AS "id!: i64",
            user_id,
            job_listing_id,
            status AS "status: Status",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        FROM application
        WHERE job_listing_id = ?
        "#,
        job_listing_id
    )
    .fetch_all(pool)
    .await?;

    Ok(applications)
}

pub async fn update_application(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
    job_listing_id: i64,
    status: Option<&Status>,
    applied_date: &str,
    cv_file_path: Option<&str>,
    cover_letter_file_path: Option<&str>,
    application_notes: Option<&str>,
) -> Result<Application, Error> {
    let status = status.map(|s| s.as_str());

    let application = query_as!(
        Application,
        r#"
        UPDATE application
        SET
            user_id = ?,
            job_listing_id = ?,
            status = ?,
            applied_date = ?,
            cv_file_path = ?,
            cover_letter_file_path = ?,
            application_notes = ?
        WHERE id = ?
        RETURNING
            id AS "id!: i64",
            user_id,
            job_listing_id,
            status AS "status: Status",
            applied_date,
            cv_file_path,
            cover_letter_file_path,
            application_notes,
            created_at,
            updated_at
        "#,
        user_id,
        job_listing_id,
        status,
        applied_date,
        cv_file_path,
        cover_letter_file_path,
        application_notes,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(application)
}

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
