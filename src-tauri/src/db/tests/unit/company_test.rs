use crate::db::models::enums::WorkType;
use crate::db::queries::company::{
    create_company, delete_company, get_all_companies, get_company_by_id, update_company,
};
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_company_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

    // 1. CREATE
    let created = create_company(
        &pool,
        "ACME Corp",
        Some("Industrial Rd 1"),
        Some("12345"),
        Some("Berlin"),
        Some("Germany"),
        Some(&WorkType::FullTime),
        Some("Manufacturing"),
        Some("https://acme.example.com"),
        Some("+49 123 456789"),
    )
    .await?;

    assert_eq!(created.name, "ACME Corp");
    assert_eq!(created.city.as_deref(), Some("Berlin"));

    // 2. READ BY ID
    let fetched = get_company_by_id(&pool, &created.id).await?;
    assert_eq!(fetched.zip_code.as_deref(), Some("12345"));

    // 3. READ ALL
    let companies = get_all_companies(&pool).await?;
    assert!(companies.len() >= 2); // includes seeded "Default Company"

    // 4. UPDATE
    let updated = update_company(
        &pool,
        &created.id,
        "ACME Updated",
        Some("Innovation St 9"),
        Some("54321"),
        Some("Munich"),
        Some("Germany"),
        Some(&WorkType::Contract),
        Some("Tech"),
        Some("https://acme-new.example.com"),
        Some("+49 987 654321"),
    )
    .await?;

    assert_eq!(updated.name, "ACME Updated");
    assert_eq!(updated.city.as_deref(), Some("Munich"));

    // 5. DELETE
    let deleted_id = delete_company(&pool, &created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
