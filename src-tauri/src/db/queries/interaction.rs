use crate::db::models::enums::InteractionType;
use crate::utils::sql_utils::build_update_sql;
use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::{query, query_as, Error, FromRow, SqlitePool};

#[derive(FromRow, Debug, Serialize)]
pub struct Interaction {
    pub id: i64,
    pub interaction_type: InteractionType,
    pub interaction_date: NaiveDate,
    pub subject: Option<String>,
    pub summary: Option<String>,
    pub medium: Option<String>,
    pub application_id: Option<i64>,
    pub person_id: Option<i64>,
    pub company_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ======================================================
// Create
// ======================================================
pub async fn create_interaction(
    pool: &SqlitePool,
    interaction_type: &InteractionType,
    interaction_date: &NaiveDate,
    subject: Option<&str>,
    summary: Option<&str>,
    medium: Option<&str>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
) -> Result<Interaction, Error> {
    let interaction_type_str = interaction_type.as_str();
    let interaction_date_str = interaction_date.format("%Y-%m-%d").to_string();

    let interaction = query_as!(
        Interaction,
        r#"
        INSERT INTO interaction (
            interaction_type,
            interaction_date,
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id AS "id!: i64",
            interaction_type AS "interaction_type: InteractionType",
            interaction_date AS "interaction_date!: NaiveDate",
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        "#,
        interaction_type_str,
        interaction_date_str,
        subject,
        summary,
        medium,
        application_id,
        person_id,
        company_id
    )
    .fetch_one(pool)
    .await?;

    Ok(interaction)
}

// ======================================================
// Get by ID
// ======================================================
pub async fn get_interaction_by_id(pool: &SqlitePool, id: i64) -> Result<Interaction, Error> {
    query_as!(
        Interaction,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_type AS "interaction_type: InteractionType",
            interaction_date AS "interaction_date!: NaiveDate",
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM interaction
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
pub async fn get_all_interactions(pool: &SqlitePool) -> Result<Vec<Interaction>, Error> {
    query_as!(
        Interaction,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_type AS "interaction_type: InteractionType",
            interaction_date AS "interaction_date!: NaiveDate",
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM interaction
        ORDER BY interaction_date DESC
        "#
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Get by Application ID
// ======================================================
pub async fn get_interactions_by_application_id(
    pool: &SqlitePool,
    application_id: i64,
) -> Result<Vec<Interaction>, Error> {
    query_as!(
        Interaction,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_type AS "interaction_type: InteractionType",
            interaction_date AS "interaction_date!: NaiveDate",
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM interaction
        WHERE application_id = ?
        ORDER BY interaction_date DESC
        "#,
        application_id
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Get by Person ID
// ======================================================
pub async fn get_interactions_by_person_id(
    pool: &SqlitePool,
    person_id: i64,
) -> Result<Vec<Interaction>, Error> {
    query_as!(
        Interaction,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_type AS "interaction_type: InteractionType",
            interaction_date AS "interaction_date!: NaiveDate",
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM interaction
        WHERE person_id = ?
        ORDER BY interaction_date DESC
        "#,
        person_id
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Get by Company ID
// ======================================================
pub async fn get_interactions_by_company_id(
    pool: &SqlitePool,
    company_id: i64,
) -> Result<Vec<Interaction>, Error> {
    query_as!(
        Interaction,
        r#"
        SELECT
            id AS "id!: i64",
            interaction_type AS "interaction_type: InteractionType",
            interaction_date AS "interaction_date!: NaiveDate",
            subject,
            summary,
            medium,
            application_id,
            person_id,
            company_id,
            created_at AS "created_at!: NaiveDateTime",
            updated_at AS "updated_at!: NaiveDateTime"
        FROM interaction
        WHERE company_id = ?
        ORDER BY interaction_date DESC
        "#,
        company_id
    )
    .fetch_all(pool)
    .await
}

// ======================================================
// Update
// ======================================================
pub async fn update_interaction(
    pool: &SqlitePool,
    id: i64,
    interaction_type: Option<&InteractionType>,
    interaction_date: Option<&NaiveDate>,
    subject: Option<&str>,
    summary: Option<&str>,
    medium: Option<&str>,
    application_id: Option<i64>,
    person_id: Option<i64>,
    company_id: Option<i64>,
) -> Result<Interaction, Error> {
    let interaction_type_str = interaction_type.map(|i| i.as_str());
    let interaction_date_s = interaction_date.map(|d| d.format("%Y-%m-%d").to_string());
    let interaction_date_ref = interaction_date_s.as_deref();

    let application_id_s = application_id.map(|v| v.to_string());
    let person_id_s = person_id.map(|v| v.to_string());
    let company_id_s = company_id.map(|v| v.to_string());

    let fields: Vec<(&str, Option<&str>)> = vec![
        ("interaction_type", interaction_type_str),
        ("interaction_date", interaction_date_ref),
        ("subject", subject),
        ("summary", summary),
        ("medium", medium),
        ("application_id", application_id_s.as_deref()),
        ("person_id", person_id_s.as_deref()),
        ("company_id", company_id_s.as_deref()),
    ];

    let (sql, binds) = build_update_sql("interaction", "id", id, fields);

    let mut query = sqlx::query_as::<_, Interaction>(&sql);
    for val in &binds {
        query = query.bind(val);
    }

    query.fetch_one(pool).await
}

// ======================================================
// Delete
// ======================================================
pub async fn delete_interaction(pool: &SqlitePool, id: i64) -> Result<i64, Error> {
    let row = query!(
        r#"
        DELETE FROM interaction
        WHERE id = ?
        RETURNING id AS "id!: i64"
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
