use crate::db::models::enums::WorkType;
use crate::utils::sql_utils::build_update_sql;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub street_address: Option<String>,
    pub zip_code: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub default_work_type: Option<WorkType>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_company(
    pool: &SqlitePool,
    name: &str,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
    default_work_type: Option<&WorkType>,
    industry: Option<&str>,
    website: Option<&str>,
    phone_number: Option<&str>,
) -> Result<Company, Error> {
    let worktype = default_work_type.map(|t| t.as_str());

    let company = query_as!(
        Company,
        r#"
        INSERT INTO company (
            name,
            street_address,
            zip_code,
            city,
            country,
            default_work_type,
            industry,
            website,
            phone_number
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            name,
            street_address,
            zip_code,
            city,
            country,
            default_work_type AS "default_work_type: WorkType",
            industry,
            website,
            phone_number,
            created_at,
            updated_at
        "#,
        name,
        street_address,
        zip_code,
        city,
        country,
        worktype,
        industry,
        website,
        phone_number
    )
    .fetch_one(pool)
    .await?;

    Ok(company)
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_company_by_id(pool: &SqlitePool, id: i64) -> Result<Company, Error> {
    let company = query_as!(
        Company,
        r#"
        SELECT
            id AS "id!: i64",
            name,
            street_address,
            zip_code,
            city,
            country,
            default_work_type AS "default_work_type: WorkType",
            industry,
            website,
            phone_number,
            created_at,
            updated_at
        FROM company
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(company)
}

// ======================================================
// Get all
// ======================================================
pub async fn get_all_companies(pool: &SqlitePool) -> Result<Vec<Company>, Error> {
    let companies = query_as!(
        Company,
        r#"
        SELECT
            id AS "id!: i64",
            name,
            street_address,
            zip_code,
            city,
            country,
            default_work_type AS "default_work_type: WorkType",
            industry,
            website,
            phone_number,
            created_at,
            updated_at
        FROM company
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(companies)
}

// ======================================================
// Update
// ======================================================
pub async fn update_company(
    pool: &SqlitePool,
    id: i64,
    name: Option<&str>,
    street_address: Option<&str>,
    zip_code: Option<&str>,
    city: Option<&str>,
    country: Option<&str>,
    default_work_type: Option<&WorkType>,
    industry: Option<&str>,
    website: Option<&str>,
    phone_number: Option<&str>,
) -> Result<Company, Error> {
    let worktype_str = default_work_type.map(|t| t.as_str());

    let fields = vec![
        ("name", name),
        ("street_address", street_address),
        ("zip_code", zip_code),
        ("city", city),
        ("country", country),
        ("default_work_type", worktype_str),
        ("industry", industry),
        ("website", website),
        ("phone_number", phone_number),
    ];

    let (sql, binds) = build_update_sql("company", "id", id, fields);

    let mut query = sqlx::query_as::<_, Company>(&sql);
    for val in binds.iter().take(binds.len() - 1) {
        query = query.bind(val);
    }
    query = query.bind(binds.last().unwrap());

    let company = query.fetch_one(pool).await?;
    Ok(company)
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_company(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM company
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
