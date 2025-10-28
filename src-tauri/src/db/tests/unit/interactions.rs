#[cfg(test)]
mod tests {
    use crate::db::models::enums::InteractionType;
    use crate::db::queries::interaction::*;
    use crate::db::tests::test_utils::setup_test_db;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_create_get_update_delete_interaction() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;
        let today = NaiveDate::from_ymd_opt(2025, 10, 26).unwrap();

        // ======================================================
        // Create
        // ======================================================
        let created = create_interaction(
            &pool,
            &InteractionType::Email,
            &today,
            Some("Follow-up Email"),
            Some("Discussed next interview step"),
            Some("video"),
            Some(1),
            Some(1),
            Some(1),
        )
        .await
        .expect("failed to create interaction");

        assert_eq!(created.interaction_type, InteractionType::Email);
        assert_eq!(created.interaction_date, today);
        assert_eq!(created.subject.as_deref(), Some("Follow-up Email"));
        assert_eq!(
            created.summary.as_deref(),
            Some("Discussed next interview step")
        );
        assert_eq!(created.medium.as_deref(), Some("video"));
        assert_eq!(created.application_id, Some(1));
        assert_eq!(created.person_id, Some(1));
        assert_eq!(created.company_id, Some(1));

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_interaction_by_id(&pool, created.id)
            .await
            .expect("failed to get interaction by id");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.interaction_type, InteractionType::Email);
        assert_eq!(fetched.subject.as_deref(), Some("Follow-up Email"));

        // ======================================================
        // Update
        // ======================================================
        let new_date = NaiveDate::from_ymd_opt(2025, 11, 5).unwrap();
        let updated = update_interaction(
            &pool,
            created.id,
            Some(&InteractionType::Phone),
            Some(&new_date),
            Some("Phone Call"),
            Some("Confirmed interview schedule"),
            Some("in_person"),
            Some(1),
            Some(1),
            Some(1),
        )
        .await
        .expect("failed to update interaction");

        assert_eq!(updated.interaction_type, InteractionType::Phone);
        assert_eq!(updated.interaction_date, new_date);
        assert_eq!(updated.subject.as_deref(), Some("Phone Call"));
        assert_eq!(
            updated.summary.as_deref(),
            Some("Confirmed interview schedule")
        );
        assert_eq!(updated.medium.as_deref(), Some("in_person"));

        // ======================================================
        // Get All
        // ======================================================
        let all = get_all_interactions(&pool)
            .await
            .expect("failed to get all interactions");
        assert!(all.iter().any(|i| i.id == updated.id));

        // ======================================================
        // Get by Person ID
        // ======================================================
        let by_person = get_interactions_by_person_id(&pool, 1)
            .await
            .expect("failed to get interactions by person id");
        assert!(by_person.iter().any(|i| i.id == updated.id));

        // ======================================================
        // Get by Application ID
        // ======================================================
        let by_app = get_interactions_by_application_id(&pool, 1)
            .await
            .expect("failed to get interactions by application id");
        assert!(by_app.iter().any(|i| i.id == updated.id));

        // ======================================================
        // Get by Company ID
        // ======================================================
        let by_company = get_interactions_by_company_id(&pool, 1)
            .await
            .expect("failed to get interactions by company id");
        assert!(by_company.iter().any(|i| i.id == updated.id));

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_interaction(&pool, updated.id)
            .await
            .expect("failed to delete interaction");
        assert_eq!(deleted_id, updated.id);

        // Confirm deletion
        let result = get_interaction_by_id(&pool, updated.id).await;
        assert!(result.is_err(), "interaction should be deleted");
    }
}
