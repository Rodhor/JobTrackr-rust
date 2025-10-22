use crate::db::models::enums::NoteType;
use crate::db::queries::contact_note::{
    create_contact_note, delete_contact_note, get_all_contact_notes, get_contact_note_by_id,
    get_contact_notes_by_contact_id, update_contact_note,
};
use crate::db::tests::test_utils::setup_test_db;

/// Ensures full CRUD functionality for `contact_note` entity works end-to-end.
/// Covers creation, retrieval (single + multi), update, and deletion.
#[tokio::test]
async fn test_contact_note_crud_flow() -> Result<(), sqlx::Error> {
    // 1. Setup isolated test database
    let pool = setup_test_db().await;

    // 2. Create
    let created = create_contact_note(&pool, 1, Some(&NoteType::Before), Some("Prep note")).await?;
    assert_eq!(created.contact_id, 1);
    assert_eq!(created.note_type, Some(NoteType::Before));
    assert_eq!(created.content.as_deref(), Some("Prep note"));

    // 3. Fetch by ID
    let fetched = get_contact_note_by_id(&pool, created.id).await?;
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.note_type, Some(NoteType::Before));

    // 4. Fetch all
    let all = get_all_contact_notes(&pool).await?;
    assert!(all.iter().any(|n| n.id == created.id));

    // 5. Fetch by contact ID
    let by_contact = get_contact_notes_by_contact_id(&pool, 1).await?;
    assert!(by_contact.iter().any(|n| n.id == created.id));

    // 6. Update (partial update test)
    let updated = update_contact_note(
        &pool,
        created.id,
        Some(1),
        Some(&NoteType::After),
        Some("Follow-up note"),
    )
    .await?;
    assert_eq!(updated.note_type, Some(NoteType::After));
    assert_eq!(updated.content.as_deref(), Some("Follow-up note"));

    // 7. Delete
    let deleted_id = delete_contact_note(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    // 8. Verify deletion
    let all_after_delete = get_all_contact_notes(&pool).await?;
    assert!(!all_after_delete.iter().any(|n| n.id == created.id));

    Ok(())
}
