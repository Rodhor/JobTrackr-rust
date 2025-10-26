#[cfg(test)]
mod tests {
    use crate::db::models::enums::{Currency, SeniorityLevel, WorkType};
    use crate::db::queries::job_listing::*;
    use crate::db::tests::test_utils::setup_test_db;

    #[tokio::test]
    async fn test_create_get_update_delete_job_listing() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;

        // ======================================================
        // Create
        // ======================================================
        let created = create_job_listing(
            &pool,
            1, // company_id from seed
            "Backend Engineer",
            Some(&WorkType::Remote),
            Some("Software"),
            Some(&SeniorityLevel::Mid),
            Some(55000),
            Some(70000),
            Some(&Currency::EUR),
            Some("Rust + SQLX developer position"),
            Some("https://jobs.example.com/backend"),
        )
        .await
        .expect("failed to create job listing");

        assert_eq!(created.company_id, 1);
        assert_eq!(created.title, "Backend Engineer");
        assert_eq!(created.work_type, Some(WorkType::Remote));
        assert_eq!(created.category.as_deref(), Some("Software"));
        assert_eq!(created.seniority_level, Some(SeniorityLevel::Mid));
        assert_eq!(created.salary_min, Some(55000));
        assert_eq!(created.salary_max, Some(70000));
        assert_eq!(created.currency, Some(Currency::EUR));
        assert_eq!(
            created.description.as_deref(),
            Some("Rust + SQLX developer position")
        );
        assert_eq!(
            created.url.as_deref(),
            Some("https://jobs.example.com/backend")
        );

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_job_listing_by_id(&pool, created.id)
            .await
            .expect("failed to get job listing by id");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.title, "Backend Engineer");
        assert_eq!(fetched.currency, Some(Currency::EUR));

        // ======================================================
        // Update
        // ======================================================
        let updated = update_job_listing(
            &pool,
            created.id,
            Some(1),
            Some("Senior Backend Engineer"),
            Some(&WorkType::Hybrid),
            Some("Engineering"),
            Some(&SeniorityLevel::Senior),
            Some(75000),
            Some(90000),
            Some(&Currency::USD),
            Some("Updated description"),
            Some("https://updated.example.com/job"),
        )
        .await
        .expect("failed to update job listing");

        assert_eq!(updated.title, "Senior Backend Engineer");
        assert_eq!(updated.work_type, Some(WorkType::Hybrid));
        assert_eq!(updated.seniority_level, Some(SeniorityLevel::Senior));
        assert_eq!(updated.salary_min, Some(75000));
        assert_eq!(updated.salary_max, Some(90000));
        assert_eq!(updated.currency, Some(Currency::USD));
        assert_eq!(updated.category.as_deref(), Some("Engineering"));
        assert_eq!(
            updated.url.as_deref(),
            Some("https://updated.example.com/job")
        );

        // ======================================================
        // Get All
        // ======================================================
        let all = get_all_job_listings(&pool)
            .await
            .expect("failed to get all job listings");
        assert!(all.iter().any(|j| j.id == updated.id));

        // ======================================================
        // Get by Company ID
        // ======================================================
        let by_company = get_job_listings_by_company_id(&pool, 1)
            .await
            .expect("failed to get job listings by company id");
        assert!(by_company.iter().any(|j| j.id == updated.id));

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_job_listing(&pool, updated.id)
            .await
            .expect("failed to delete job listing");
        assert_eq!(deleted_id, updated.id);

        // Confirm deletion
        let result = get_job_listing_by_id(&pool, updated.id).await;
        assert!(result.is_err(), "job listing should be deleted");
    }
}
