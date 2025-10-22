use crate::db::models::enums::WorkType;
use crate::db::queries::company::{
    create_company, delete_company, get_all_companies, get_company_by_id, update_company,
};
use crate::db::tests::test_utils::setup_test_db;

/// Full CRUD integration test for the `company` table.
#[tokio::test]
async fn test_company_crud_flow() -> Result<(), sqlx::Error> {
    // 1. Setup isolated in-memory DB
    let pool = setup_test_db().await;

    // 2. CREATE
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
    assert_eq!(created.default_work_type, Some(WorkType::FullTime));

    // 3. READ BY ID
    let fetched = get_company_by_id(&pool, created.id).await?;
    assert_eq!(fetched.zip_code.as_deref(), Some("12345"));
    assert_eq!(fetched.country.as_deref(), Some("Germany"));

    // 4. READ ALL
    let companies = get_all_companies(&pool).await?;
    assert!(companies.iter().any(|c| c.id == created.id));

    // 5. UPDATE (partial)
    let updated = update_company(
        &pool,
        created.id,
        Some("ACME Updated"),
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
    assert_eq!(updated.default_work_type, Some(WorkType::Contract));

    // 6. DELETE
    let deleted_id = delete_company(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    // 7. VERIFY DELETION
    let remaining = get_all_companies(&pool).await?;
    assert!(!remaining.iter().any(|c| c.id == created.id));

    Ok(())
}
