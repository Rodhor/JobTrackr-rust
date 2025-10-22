use crate::db::models::enums::Status;
use crate::db::queries::application::{
    create_application, delete_application, get_all_applications, get_application_by_id,
    get_applications_by_job_listing_id, get_applications_by_user_id, update_application,
};
use crate::db::tests::test_utils::setup_test_db;

/// Ensures that CRUD operations for `application` work correctly end-to-end.
/// Covers create, read (by id, user, and job listing), update, and delete.
#[tokio::test]
async fn test_application_crud_flow() -> Result<(), sqlx::Error> {
    // 1. Setup isolated in-memory test database
    let pool = setup_test_db().await;

    // 2. Create
    let created = create_application(
        &pool,
        1,
        1,
        Some(&Status::Applied),
        "2025-01-01T00:00:00Z",
        Some("/tmp/cv.pdf"),
        Some("/tmp/cover.pdf"),
        Some("Initial note"),
    )
    .await?;

    assert_eq!(created.user_id, 1);
    assert_eq!(created.job_listing_id, 1);
    assert_eq!(created.status, Some(Status::Applied));
    assert_eq!(created.application_notes.as_deref(), Some("Initial note"));

    // 3. Fetch by ID
    let fetched = get_application_by_id(&pool, created.id).await?;
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.cv_file_path.as_deref(), Some("/tmp/cv.pdf"));

    // 4. Fetch all
    let all = get_all_applications(&pool).await?;
    assert!(all.iter().any(|a| a.id == created.id));

    // 5. Fetch by user ID
    let by_user = get_applications_by_user_id(&pool, 1).await?;
    assert!(by_user.iter().any(|a| a.id == created.id));

    // 6. Fetch by job listing ID
    let by_listing = get_applications_by_job_listing_id(&pool, 1).await?;
    assert!(by_listing.iter().any(|a| a.id == created.id));

    // 7. Update (partial dynamic update test)
    let updated = update_application(
        &pool,
        created.id,
        None,                           // keep user_id unchanged
        None,                           // keep job_listing_id unchanged
        Some(&Status::Interviewing),    // update status
        Some("2025-02-02T00:00:00Z"),   // update applied_date
        Some("/tmp/cv_updated.pdf"),    // update cv
        Some("/tmp/cover_updated.pdf"), // update cover letter
        Some("Updated note"),           // update note
    )
    .await?;

    assert_eq!(updated.status, Some(Status::Interviewing));
    assert_eq!(updated.cv_file_path.as_deref(), Some("/tmp/cv_updated.pdf"));
    assert_eq!(updated.application_notes.as_deref(), Some("Updated note"));

    // 8. Delete
    let deleted_id = delete_application(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    // 9. Verify deletion
    let remaining = get_all_applications(&pool).await?;
    assert!(!remaining.iter().any(|a| a.id == created.id));

    Ok(())
}
