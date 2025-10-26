#[cfg(test)]
mod tests {
    use crate::db::models::enums::WorkType;
    use crate::db::queries::company::*;
    use crate::db::tests::test_utils::setup_test_db;

    #[tokio::test]
    async fn test_create_get_update_delete_company() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;

        // ======================================================
        // Create
        // ======================================================
        let created = create_company(
            &pool,
            "Acme Corp",
            Some("Main Street 1"),
            Some("12345"),
            Some("Berlin"),
            Some("Germany"),
            Some(&WorkType::Remote),
            Some("Software"),
            Some("https://acme.test"),
            Some("+4912345678"),
        )
        .await
        .expect("failed to create company");

        assert_eq!(created.name, "Acme Corp");
        assert_eq!(created.city.as_deref(), Some("Berlin"));
        assert_eq!(created.country.as_deref(), Some("Germany"));
        assert_eq!(created.default_work_type, Some(WorkType::Remote));
        assert_eq!(created.industry.as_deref(), Some("Software"));
        assert_eq!(created.website.as_deref(), Some("https://acme.test"));
        assert_eq!(created.phone_number.as_deref(), Some("+4912345678"));

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_company_by_id(&pool, created.id)
            .await
            .expect("failed to get company by id");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "Acme Corp");
        assert_eq!(fetched.default_work_type, Some(WorkType::Remote));

        // ======================================================
        // Update
        // ======================================================
        let updated = update_company(
            &pool,
            created.id,
            Some("Updated Corp"),
            Some("Updated Street 9"),
            Some("99999"),
            Some("Hamburg"),
            Some("Germany"),
            Some(&WorkType::Remote),
            Some("Consulting"),
            Some("https://updated.test"),
            Some("+4900000000"),
        )
        .await
        .expect("failed to update company");

        assert_eq!(updated.name, "Updated Corp");
        assert_eq!(updated.city.as_deref(), Some("Hamburg"));
        assert_eq!(updated.default_work_type, Some(WorkType::Remote));
        assert_eq!(updated.industry.as_deref(), Some("Consulting"));
        assert_eq!(updated.website.as_deref(), Some("https://updated.test"));

        // ======================================================
        // Get All
        // ======================================================
        let all = get_all_companies(&pool)
            .await
            .expect("failed to get all companies");
        assert!(all.iter().any(|c| c.id == updated.id));

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_company(&pool, updated.id)
            .await
            .expect("failed to delete company");
        assert_eq!(deleted_id, updated.id);

        // Confirm deletion
        let result = get_company_by_id(&pool, updated.id).await;
        assert!(result.is_err(), "company should be deleted");
    }
}
