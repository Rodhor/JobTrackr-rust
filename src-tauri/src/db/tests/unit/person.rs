#[cfg(test)]
mod tests {
    use crate::db::models::enums::Role;
    use crate::db::queries::person::*;
    use crate::db::tests::test_utils::setup_test_db;

    #[tokio::test]
    async fn test_create_get_update_delete_person() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;

        // ======================================================
        // Create
        // ======================================================
        let created = create_person(
            &pool,
            "Alice",
            "Johnson",
            Some("alice@example.com"),
            Some("+49123456789"),
            Some(&Role::Recruiter),
            Some("https://linkedin.com/in/alicejohnson"),
            Some(1),
        )
        .await
        .expect("failed to create person");

        assert_eq!(created.first_name, "Alice");
        assert_eq!(created.last_name, "Johnson");
        assert_eq!(created.email.as_deref(), Some("alice@example.com"));
        assert_eq!(created.phone_number.as_deref(), Some("+49123456789"));
        assert_eq!(created.role, Some(Role::Recruiter));
        assert_eq!(
            created.linkedin_url.as_deref(),
            Some("https://linkedin.com/in/alicejohnson")
        );
        assert_eq!(created.company_id, Some(1));

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_person_by_id(&pool, created.id)
            .await
            .expect("failed to get person by id");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.first_name, "Alice");
        assert_eq!(fetched.role, Some(Role::Recruiter));

        // ======================================================
        // Update
        // ======================================================
        let updated = update_person(
            &pool,
            created.id,
            Some("Alice-Marie"),
            Some("Johnson"),
            Some("alice@updated.com"),
            Some("+4999999999"),
            Some(&Role::HR),
            Some("https://linkedin.com/in/alice-marie"),
            Some(1),
        )
        .await
        .expect("failed to update person");

        assert_eq!(updated.first_name, "Alice-Marie");
        assert_eq!(updated.role, Some(Role::HR));
        assert_eq!(updated.email.as_deref(), Some("alice@updated.com"));
        assert_eq!(updated.phone_number.as_deref(), Some("+4999999999"));
        assert_eq!(
            updated.linkedin_url.as_deref(),
            Some("https://linkedin.com/in/alice-marie")
        );

        // ======================================================
        // Get All
        // ======================================================
        let all = get_all_persons(&pool)
            .await
            .expect("failed to get all persons");
        assert!(all.iter().any(|p| p.id == updated.id));

        // ======================================================
        // Get by Company ID
        // ======================================================
        let by_company = get_persons_by_company_id(&pool, 1)
            .await
            .expect("failed to get persons by company id");
        assert!(by_company.iter().any(|p| p.id == updated.id));

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_person(&pool, updated.id)
            .await
            .expect("failed to delete person");
        assert_eq!(deleted_id, updated.id);

        // Confirm deletion
        let result = get_person_by_id(&pool, updated.id).await;
        assert!(result.is_err(), "person should be deleted");
    }
}
