#[cfg(test)]
mod tests {
    use crate::db::models::enums::Stage;
    use crate::db::queries::application::*;
    use crate::db::tests::test_utils::setup_test_db;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_create_get_update_delete_application() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;
        let today = NaiveDate::from_ymd_opt(2025, 10, 26).unwrap();

        // ======================================================
        // Create
        // ======================================================
        let created = create_application(
            &pool,
            Some(1),
            Some(&Stage::Applied),
            &today,
            Some("/tmp/cv.pdf"),
            Some("/tmp/cover.pdf"),
            Some("Initial note"),
        )
        .await
        .expect("failed to create application");

        assert_eq!(created.job_listing_id, Some(1));
        assert_eq!(created.stage, Some(Stage::Applied));
        assert_eq!(created.applied_date, today);
        assert_eq!(created.cv_file_path.as_deref(), Some("/tmp/cv.pdf"));
        assert_eq!(
            created.cover_letter_file_path.as_deref(),
            Some("/tmp/cover.pdf")
        );
        assert_eq!(created.application_notes.as_deref(), Some("Initial note"));

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_application_by_id(&pool, created.id)
            .await
            .expect("failed to get application by id");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.stage, Some(Stage::Applied));
        assert_eq!(fetched.applied_date, today);

        // ======================================================
        // Update
        // ======================================================
        let new_date = NaiveDate::from_ymd_opt(2025, 11, 1).unwrap();
        let updated = update_application(
            &pool,
            created.id,
            Some(1),
            Some(&Stage::Interviewing),
            Some(&new_date),
            Some("/tmp/new_cv.pdf"),
            Some("/tmp/new_cover.pdf"),
            Some("Updated note"),
        )
        .await
        .expect("failed to update application");

        assert_eq!(updated.stage, Some(Stage::Interviewing));
        assert_eq!(updated.applied_date, new_date);
        assert_eq!(updated.cv_file_path.as_deref(), Some("/tmp/new_cv.pdf"));
        assert_eq!(
            updated.cover_letter_file_path.as_deref(),
            Some("/tmp/new_cover.pdf")
        );
        assert_eq!(updated.application_notes.as_deref(), Some("Updated note"));

        // ======================================================
        // Get All
        // ======================================================
        let all = get_all_applications(&pool)
            .await
            .expect("failed to get all applications");
        assert!(all.iter().any(|a| a.id == updated.id));

        // ======================================================
        // Get by Job Listing ID
        // ======================================================
        let by_job = get_applications_by_job_listing_id(&pool, 1)
            .await
            .expect("failed to get by job listing");
        assert!(by_job.iter().any(|a| a.id == updated.id));

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_application(&pool, created.id)
            .await
            .expect("failed to delete application");
        assert_eq!(deleted_id, created.id);

        let result = get_application_by_id(&pool, created.id).await;
        assert!(result.is_err(), "application should be deleted");
    }
}
