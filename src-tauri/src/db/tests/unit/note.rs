#[cfg(test)]
mod tests {
    use crate::db::models::enums::NoteType;
    use crate::db::queries::note::*;
    use crate::db::tests::test_utils::setup_test_db;

    #[tokio::test]
    async fn test_create_get_update_delete_note() {
        // ======================================================
        // Setup
        // ======================================================
        let pool = setup_test_db().await;

        // ======================================================
        // Create
        // ======================================================
        let created = create_note(
            &pool,
            Some(1), // interaction_id from seed
            Some(1), // job_listing_id from seed
            Some(1), // application_id from seed
            Some(1), // person_id from seed
            Some(1), // company_id from seed
            Some(&NoteType::General),
            Some("Initial note title"),
            Some("Initial note content"),
        )
        .await
        .expect("failed to create note");

        assert_eq!(created.interaction_id, Some(1));
        assert_eq!(created.job_listing_id, Some(1));
        assert_eq!(created.application_id, Some(1));
        assert_eq!(created.person_id, Some(1));
        assert_eq!(created.company_id, Some(1));
        assert_eq!(created.note_type, Some(NoteType::General));
        assert_eq!(created.title.as_deref(), Some("Initial note title"));
        assert_eq!(created.content.as_deref(), Some("Initial note content"));

        // ======================================================
        // Get by ID
        // ======================================================
        let fetched = get_note_by_id(&pool, created.id)
            .await
            .expect("failed to get note by id");

        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.note_type, Some(NoteType::General));
        assert_eq!(fetched.content.as_deref(), Some("Initial note content"));
        assert_eq!(fetched.title.as_deref(), Some("Initial note title"));

        // ======================================================
        // Update
        // ======================================================
        let updated = update_note(
            &pool,
            created.id,
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(&NoteType::Feedback),
            Some("Updated title"),
            Some("Updated content"),
        )
        .await
        .expect("failed to update note");

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.note_type, Some(NoteType::Feedback));
        assert_eq!(updated.title.as_deref(), Some("Updated title"));
        assert_eq!(updated.content.as_deref(), Some("Updated content"));

        // ======================================================
        // Get All
        // ======================================================
        let all = get_all_notes(&pool).await.expect("failed to get all notes");
        assert!(all.iter().any(|n| n.id == updated.id));

        // ======================================================
        // Get by Application ID
        // ======================================================
        let by_app = get_notes_by_application_id(&pool, 1)
            .await
            .expect("failed to get notes by application");
        assert!(by_app.iter().any(|n| n.id == updated.id));

        // ======================================================
        // Get by Interaction ID
        // ======================================================
        let by_interaction = get_notes_by_interaction_id(&pool, 1)
            .await
            .expect("failed to get notes by interaction");
        assert!(by_interaction.iter().any(|n| n.id == updated.id));

        // ======================================================
        // Delete
        // ======================================================
        let deleted_id = delete_note(&pool, updated.id)
            .await
            .expect("failed to delete note");
        assert_eq!(deleted_id, updated.id);

        // Confirm deletion
        let result = get_note_by_id(&pool, updated.id).await;
        assert!(result.is_err(), "note should be deleted");
    }
}
