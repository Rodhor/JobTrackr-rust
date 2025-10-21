use crate::db::enums::WorkType;
use chrono::Utc;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug)]
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
    pub created_at: String,
    pub updated_at: String,
}

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
    let now = Utc::now().to_rfc3339();
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
            phone_number,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        phone_number,
        now,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(company)
}

pub async fn get_company_by_id(pool: &SqlitePool, id: &i64) -> Result<Company, Error> {
    let company = query_as!(
        Company,
        r#"
        SELECT
            id,
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

pub async fn get_all_companies(pool: &SqlitePool) -> Result<Vec<Company>, Error> {
    let companies = query_as!(
        Company,
        r#"
        SELECT
            id,
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

pub async fn update_company(
    pool: &SqlitePool,
    id: &i64,
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
    let now = Utc::now().to_rfc3339();
    let worktype = default_work_type.map(|t| t.as_str());

    let company = query_as!(
        Company,
        r#"
        UPDATE company
        SET
            name = ?,
            street_address = ?,
            zip_code = ?,
            city = ?,
            country = ?,
            default_work_type = ?,
            industry = ?,
            website = ?,
            phone_number = ?,
            updated_at = ?
        WHERE id = ?
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
        phone_number,
        now,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(company)
}

pub async fn delete_company(pool: &SqlitePool, id: &i64) -> Result<i64, Error> {
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
