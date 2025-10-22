use crate::db::models::enums::NoteType;
use crate::db::queries::contact_note::{
    create_contact_note, delete_contact_note, get_all_contact_notes, get_contact_note_by_id,
    get_contact_notes_by_contact_id, update_contact_note,
};
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_contact_note_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

    let created = create_contact_note(&pool, 1, Some(&NoteType::Before), Some("Prep note")).await?;
    assert_eq!(created.note_type, Some(NoteType::Before));

    let fetched = get_contact_note_by_id(&pool, created.id).await?;
    assert_eq!(fetched.content.as_deref(), Some("Prep note"));

    let all = get_all_contact_notes(&pool).await?;
    assert!(all.len() >= 1);

    let by_contact = get_contact_notes_by_contact_id(&pool, 1).await?;
    assert!(by_contact.len() >= 1);

    let updated = update_contact_note(
        &pool,
        created.id,
        1,
        Some(&NoteType::After),
        Some("Follow-up"),
    )
    .await?;
    assert_eq!(updated.note_type, Some(NoteType::After));
    assert_eq!(updated.content.as_deref(), Some("Follow-up"));

    let deleted_id = delete_contact_note(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
