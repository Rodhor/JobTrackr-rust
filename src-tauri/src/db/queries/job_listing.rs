use crate::db::models::enums::{Currency, SeniorityLevel, WorkType};
use crate::utils::sql_utils::build_update_sql;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
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
    let work_type_str = work_type.map(|t| t.as_str());
    let seniority_level_str = seniority_level.map(|l| l.as_str());
    let currency_str = currency.map(|c| c.as_str());

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
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        "#,
        company_id,
        title,
        work_type_str,
        category,
        seniority_level_str,
        salary_min,
        salary_max,
        currency_str,
        description,
        url
    )
    .fetch_one(pool)
    .await?;

    Ok(job_listing)
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_job_listing_by_id(pool: &SqlitePool, id: i64) -> Result<JobListing, Error> {
    query_as!(
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
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM job_listing
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
pub async fn get_all_job_listings(pool: &SqlitePool) -> Result<Vec<JobListing>, Error> {
    query_as!(
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
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM job_listing
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Get by Company ID
// ======================================================
pub async fn get_job_listings_by_company_id(
    pool: &SqlitePool,
    company_id: i64,
) -> Result<Vec<JobListing>, Error> {
    query_as!(
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
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM job_listing
        WHERE company_id = ?
        ORDER BY created_at DESC
        "#,
        company_id
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Update
// ======================================================
pub async fn update_job_listing(
    pool: &SqlitePool,
    id: i64,
    company_id: Option<i64>,
    title: Option<&str>,
    work_type: Option<&WorkType>,
    category: Option<&str>,
    seniority_level: Option<&SeniorityLevel>,
    salary_min: Option<i64>,
    salary_max: Option<i64>,
    currency: Option<&Currency>,
    description: Option<&str>,
    url: Option<&str>,
) -> Result<JobListing, Error> {
    let work_type_str = work_type.map(|t| t.as_str());
    let seniority_level_str = seniority_level.map(|l| l.as_str());
    let currency_str = currency.map(|c| c.as_str());

    let company_id_s = company_id.map(|v| v.to_string());
    let salary_min_s = salary_min.map(|v| v.to_string());
    let salary_max_s = salary_max.map(|v| v.to_string());

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("company_id", company_id_s.as_deref()),
        ("title", title),
        ("work_type", work_type_str),
        ("category", category),
        ("seniority_level", seniority_level_str),
        ("salary_min", salary_min_s.as_deref()),
        ("salary_max", salary_max_s.as_deref()),
        ("currency", currency_str),
        ("description", description),
        ("url", url),
    ];

    let (sql, binds) = build_update_sql("job_listing", "id", id, fields);

    let mut query = sqlx::query_as::<_, JobListing>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    query.fetch_one(pool).await
}

// ======================================================
// Delete
// ======================================================
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
