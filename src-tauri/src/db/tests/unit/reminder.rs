#[cfg(test)]
mod tests {
    use crate::db::queries::reminder::*;
    use crate::db::tests::test_utils::setup_test_db;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_create_get_update_delete_reminder() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;

        let reminder_date = NaiveDate::from_ymd_opt(2025, 10, 30).unwrap();

        // ======================================================
        // Create
        // ======================================================
        let created = create_reminder(
            &pool,
            Some(1), // application_id
            Some(1), // contact_id
            Some(1), // note_id
            Some(1), // job_listing_id
            Some(1), // company_id
            Some(1), // person_id
            &reminder_date,
            Some("Follow up interview"),
            Some("Reach out to recruiter about next steps"),
            false,
        )
        .await
        .expect("failed to create reminder");

        assert_eq!(created.application_id, Some(1));
        assert_eq!(created.contact_id, Some(1));
        assert_eq!(created.note_id, Some(1));
        assert_eq!(created.job_listing_id, Some(1));
        assert_eq!(created.company_id, Some(1));
        assert_eq!(created.person_id, Some(1));
        assert_eq!(created.reminder_date, reminder_date);
        assert_eq!(created.title.as_deref(), Some("Follow up interview"));
        assert_eq!(
            created.message.as_deref(),
            Some("Reach out to recruiter about next steps")
        );
        assert!(!created.is_completed);

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_reminder_by_id(&pool, created.id)
            .await
            .expect("failed to fetch reminder by id");
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.is_completed, false);

        // ======================================================
        // Get all
        // ======================================================
        let all = get_all_reminders(&pool)
            .await
            .expect("failed to get all reminders");
        assert!(all.iter().any(|r| r.id == created.id));

        // ======================================================
        // Get upcoming (should include this reminder)
        // ======================================================
        let today = NaiveDate::from_ymd_opt(2025, 10, 25).unwrap();
        let upcoming = get_upcoming_reminders(&pool, &today)
            .await
            .expect("failed to get upcoming reminders");

        assert!(
            upcoming.iter().any(|r| r.id == created.id),
            "created reminder should be included in upcoming list"
        );

        // ======================================================
        // Update
        // ======================================================
        let new_date = NaiveDate::from_ymd_opt(2025, 11, 5).unwrap();
        let updated = update_reminder(
            &pool,
            created.id,
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(&new_date),
            Some("Final Interview"),
            Some("Confirm meeting slot"),
            Some(true),
        )
        .await
        .expect("failed to update reminder");

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.reminder_date, new_date);
        assert_eq!(updated.title.as_deref(), Some("Final Interview"));
        assert_eq!(updated.message.as_deref(), Some("Confirm meeting slot"));
        assert!(updated.is_completed);

        // ======================================================
        // Get upcoming (should NOT include updated reminder since completed)
        // ======================================================
        let upcoming_after = get_upcoming_reminders(&pool, &today)
            .await
            .expect("failed to get upcoming reminders after update");

        assert!(
            !upcoming_after.iter().any(|r| r.id == updated.id),
            "completed reminder should not appear in upcoming list"
        );

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_reminder(&pool, updated.id)
            .await
            .expect("failed to delete reminder");
        assert_eq!(deleted_id, updated.id);

        // Confirm deletion
        let result = get_reminder_by_id(&pool, updated.id).await;
        assert!(result.is_err(), "reminder should be deleted");
    }
}
