use crate::db::models::enums::{Currency, SeniorityLevel, WorkType};
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
pub struct JobListing {
    pub id: i64,
    pub company_id: i64,
    pub title: String,
    pub work_type: Option<WorkType>,
    pub category: Option<String>,
    pub seniority_level: Option<SeniorityLevel>,
    pub salary_min: Option<i64>,
    pub salary_max: Option<i64>,
    pub currency: Option<Currency>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_job_listing(
    pool: &SqlitePool,
    company_id: i64,
    title: &str,
    work_type: Option<&WorkType>,
    category: Option<&str>,
    seniority_level: Option<&SeniorityLevel>,
    salary_min: Option<i64>,
    salary_max: Option<i64>,
    currency: Option<&Currency>,
    description: Option<&str>,
    url: Option<&str>,
) -> Result<JobListing, Error> {
    let work_type = work_type.map(|t| t.as_str());
    let seniority_level = seniority_level.map(|l| l.as_str());
    let currency = currency.map(|c| c.as_str());

    let job_listing = query_as!(
        JobListing,
        r#"
        INSERT INTO job_listing (
            company_id,
            title,
            work_type,
            category,
            seniority_level,
            salary_min,
            salary_max,
            currency,
            description,
            url
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            company_id,
            title,
            work_type AS "work_type: WorkType",
            category,
            seniority_level AS "seniority_level: SeniorityLevel",
            salary_min,
            salary_max,
            currency AS "currency: Currency",
            description,
            url,
            created_at,
            updated_at
        "#,
        company_id,
        title,
        work_type,
        category,
        seniority_level,
        salary_min,
        salary_max,
        currency,
        description,
        url
    )
    .fetch_one(pool)
    .await?;

    Ok(job_listing)
}

pub async fn get_job_listing_by_id(pool: &SqlitePool, id: i64) -> Result<JobListing, Error> {
    let job_listing = query_as!(
        JobListing,
        r#"
        SELECT
            id AS "id!: i64",
            company_id,
            title,
            work_type AS "work_type: WorkType",
            category,
            seniority_level AS "seniority_level: SeniorityLevel",
            salary_min,
            salary_max,
            currency AS "currency: Currency",
            description,
            url,
            created_at,
            updated_at
        FROM job_listing
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(job_listing)
}

pub async fn get_all_job_listings(pool: &SqlitePool) -> Result<Vec<JobListing>, Error> {
    let listings = query_as!(
        JobListing,
        r#"
        SELECT
            id AS "id!: i64",
            company_id,
            title,
            work_type AS "work_type: WorkType",
            category,
            seniority_level AS "seniority_level: SeniorityLevel",
            salary_min,
            salary_max,
            currency AS "currency: Currency",
            description,
            url,
            created_at,
            updated_at
        FROM job_listing
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(listings)
}

pub async fn get_job_listings_by_company_id(
    pool: &SqlitePool,
    company_id: i64,
) -> Result<Vec<JobListing>, Error> {
    let listings = query_as!(
        JobListing,
        r#"
        SELECT
            id AS "id!: i64",
            company_id,
            title,
            work_type AS "work_type: WorkType",
            category,
            seniority_level AS "seniority_level: SeniorityLevel",
            salary_min,
            salary_max,
            currency AS "currency: Currency",
            description,
            url,
            created_at,
            updated_at
        FROM job_listing
        WHERE company_id = ?
        "#,
        company_id
    )
    .fetch_all(pool)
    .await?;

    Ok(listings)
}

pub async fn update_job_listing(
    pool: &SqlitePool,
    id: i64,
    company_id: i64,
    title: &str,
    work_type: Option<&WorkType>,
    category: Option<&str>,
    seniority_level: Option<&SeniorityLevel>,
    salary_min: Option<i64>,
    salary_max: Option<i64>,
    currency: Option<&Currency>,
    description: Option<&str>,
    url: Option<&str>,
) -> Result<JobListing, Error> {
    let work_type = work_type.map(|t| t.as_str());
    let seniority_level = seniority_level.map(|l| l.as_str());
    let currency = currency.map(|c| c.as_str());

    let job_listing = query_as!(
        JobListing,
        r#"
        UPDATE job_listing
        SET
            company_id = ?,
            title = ?,
            work_type = ?,
            category = ?,
            seniority_level = ?,
            salary_min = ?,
            salary_max = ?,
            currency = ?,
            description = ?,
            url = ?
        WHERE id = ?
        RETURNING
            id AS "id!: i64",
            company_id,
            title,
            work_type AS "work_type: WorkType",
            category,
            seniority_level AS "seniority_level: SeniorityLevel",
            salary_min,
            salary_max,
            currency AS "currency: Currency",
            description,
            url,
            created_at,
            updated_at
        "#,
        company_id,
        title,
        work_type,
        category,
        seniority_level,
        salary_min,
        salary_max,
        currency,
        description,
        url,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(job_listing)
}

pub async fn delete_job_listing(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM job_listing
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
