use crate::db::models::enums::{Currency, SeniorityLevel, WorkType};
use crate::db::queries::job_listing::{
    create_job_listing, delete_job_listing, get_all_job_listings, get_job_listing_by_id,
    get_job_listings_by_company_id, update_job_listing,
};
use crate::db::tests::test_utils::setup_test_db;

/// Full CRUD integration test for `job_listing` table.
#[tokio::test]
async fn test_job_listing_crud_flow() -> Result<(), sqlx::Error> {
    // 1. Setup isolated in-memory DB
    let pool = setup_test_db().await;

    // 2. CREATE
    let created = create_job_listing(
        &pool,
        1, // company_id
        "Software Engineer",
        Some(&WorkType::FullTime),
        Some("Engineering"),
        Some(&SeniorityLevel::Mid),
        Some(60000),
        Some(90000),
        Some(&Currency::EUR),
        Some("Develop backend services"),
        Some("https://example.com/jobs/1"),
    )
    .await?;

    assert_eq!(created.title, "Software Engineer");
    assert_eq!(created.company_id, 1);
    assert_eq!(created.work_type, Some(WorkType::FullTime));
    assert_eq!(created.seniority_level, Some(SeniorityLevel::Mid));
    assert_eq!(created.currency, Some(Currency::EUR));

    // 3. READ BY ID
    let fetched = get_job_listing_by_id(&pool, created.id).await?;
    assert_eq!(fetched.title, "Software Engineer");
    assert_eq!(fetched.company_id, created.company_id);

    // 4. READ ALL
    let all = get_all_job_listings(&pool).await?;
    assert!(all.iter().any(|j| j.id == created.id));

    // 5. READ BY COMPANY
    let by_company = get_job_listings_by_company_id(&pool, 1).await?;
    assert!(by_company.iter().any(|j| j.id == created.id));

    // 6. UPDATE (partial)
    let updated = update_job_listing(
        &pool,
        created.id,
        Some(1),                          // same company
        Some("Backend Developer"),        // new title
        Some(&WorkType::Freelance),       // new work type
        Some("Platform Team"),            // new category
        Some(&SeniorityLevel::Senior),    // new seniority
        Some(75000),                      // new salary min
        Some(110000),                     // new salary max
        Some(&Currency::USD),             // new currency
        Some("Lead backend systems dev"), // new description
        Some("https://example.com/jobs/updated"),
    )
    .await?;

    assert_eq!(updated.title, "Backend Developer");
    assert_eq!(updated.work_type, Some(WorkType::Freelance));
    assert_eq!(updated.seniority_level, Some(SeniorityLevel::Senior));
    assert_eq!(updated.currency, Some(Currency::USD));
    assert_eq!(updated.salary_min, Some(75000));

    // 7. DELETE
    let deleted_id = delete_job_listing(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    // 8. VERIFY DELETION
    let remaining = get_all_job_listings(&pool).await?;
    assert!(!remaining.iter().any(|j| j.id == created.id));

    Ok(())
}
