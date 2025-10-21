use crate::db::application::{
    create_application, delete_application, get_all_applications, get_application_by_id,
    get_applications_by_job_listing_id, get_applications_by_user_id, update_application,
};
use crate::db::enums::Status;
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_application_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

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
    assert_eq!(created.status, Some(Status::Applied));

    let fetched = get_application_by_id(&pool, created.id).await?;
    assert_eq!(fetched.cv_file_path.as_deref(), Some("/tmp/cv.pdf"));

    let all = get_all_applications(&pool).await?;
    assert_eq!(all.len(), 1);

    let by_user = get_applications_by_user_id(&pool, 1).await?;
    assert_eq!(by_user.len(), 1);

    let by_listing = get_applications_by_job_listing_id(&pool, 1).await?;
    assert_eq!(by_listing.len(), 1);

    let updated = update_application(
        &pool,
        created.id,
        1,
        1,
        Some(&Status::Interviewing),
        "2025-02-02T00:00:00Z",
        Some("/tmp/cv_new.pdf"),
        Some("/tmp/cover_new.pdf"),
        Some("Updated note"),
    )
    .await?;
    assert_eq!(updated.status, Some(Status::Interviewing));

    let deleted_id = delete_application(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
