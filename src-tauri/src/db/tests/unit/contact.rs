#[cfg(test)]
mod tests {
    use crate::db::models::enums::ContactType;
    use crate::db::queries::contact::*;
    use crate::db::tests::test_utils::setup_test_db;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_create_get_update_delete_contact() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;
        let today = NaiveDate::from_ymd_opt(2025, 10, 26).unwrap();

        // ======================================================
        // Create
        // ======================================================
        let created = create_contact(
            &pool,
            &ContactType::Email,
            &today,
            Some("Berlin HQ"),
            Some(1),
            Some(1),
        )
        .await
        .expect("failed to create contact");

        assert_eq!(created.contact_type, ContactType::Email);
        assert_eq!(created.contact_date, today);
        assert_eq!(created.location.as_deref(), Some("Berlin HQ"));
        assert_eq!(created.person_id, Some(1));
        assert_eq!(created.application_id, Some(1));

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_contact_by_id(&pool, created.id)
            .await
            .expect("failed to get contact by id");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.contact_type, ContactType::Email);
        assert_eq!(fetched.location.as_deref(), Some("Berlin HQ"));

        // ======================================================
        // Update
        // ======================================================
        let new_date = NaiveDate::from_ymd_opt(2025, 11, 5).unwrap();
        let updated = update_contact(
            &pool,
            created.id,
            Some(&ContactType::Phone),
            Some(&new_date),
            Some("Hamburg Office"),
            Some(1),
            Some(1),
        )
        .await
        .expect("failed to update contact");

        assert_eq!(updated.contact_type, ContactType::Phone);
        assert_eq!(updated.contact_date, new_date);
        assert_eq!(updated.location.as_deref(), Some("Hamburg Office"));

        // ======================================================
        // Get All
        // ======================================================
        let all = get_all_contacts(&pool)
            .await
            .expect("failed to get all contacts");
        assert!(all.iter().any(|c| c.id == updated.id));

        // ======================================================
        // Get by Person ID
        // ======================================================
        let by_person = get_contacts_by_person_id(&pool, 1)
            .await
            .expect("failed to get contacts by person id");
        assert!(by_person.iter().any(|c| c.id == updated.id));

        // ======================================================
        // Get by Application ID
        // ======================================================
        let by_app = get_contacts_by_application_id(&pool, 1)
            .await
            .expect("failed to get contacts by application id");
        assert!(by_app.iter().any(|c| c.id == updated.id));

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_contact(&pool, updated.id)
            .await
            .expect("failed to delete contact");
        assert_eq!(deleted_id, updated.id);

        // Confirm deletion
        let result = get_contact_by_id(&pool, updated.id).await;
        assert!(result.is_err(), "contact should be deleted");
    }
}
