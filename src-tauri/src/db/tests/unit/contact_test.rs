use crate::db::models::enums::ContactType;
use crate::db::queries::contact::{
    create_contact, delete_contact, get_all_contacts, get_contact_by_id, get_contacts_by_person_id,
    get_contacts_by_user_id, update_contact,
};
use crate::db::tests::test_utils::setup_test_db;

/// Validates CRUD functionality for `contact` table end-to-end.
#[tokio::test]
async fn test_contact_crud_flow() -> Result<(), sqlx::Error> {
    // 1. Setup isolated test DB
    let pool = setup_test_db().await;

    // 2. CREATE
    let created = create_contact(
        &pool,
        &ContactType::Email,
        "2025-01-01T00:00:00Z",
        Some("Berlin"),
        1,
        Some(1),
    )
    .await?;

    assert_eq!(created.contact_type, ContactType::Email);
    assert_eq!(created.location.as_deref(), Some("Berlin"));
    assert_eq!(created.user_id, 1);

    // 3. READ BY ID
    let fetched = get_contact_by_id(&pool, created.id).await?;
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.contact_type, ContactType::Email);

    // 4. READ ALL
    let all = get_all_contacts(&pool).await?;
    assert!(all.iter().any(|c| c.id == created.id));

    // 5. READ BY USER
    let by_user = get_contacts_by_user_id(&pool, 1).await?;
    assert!(by_user.iter().any(|c| c.id == created.id));

    // 6. READ BY PERSON
    let by_person = get_contacts_by_person_id(&pool, 1).await?;
    assert!(by_person.iter().any(|c| c.id == created.id));

    // 7. UPDATE (partial)
    let updated = update_contact(
        &pool,
        created.id,
        Some(&ContactType::Phone),    // changed type
        Some("2025-02-02T00:00:00Z"), // new date
        Some("Munich"),               // new location
        None,                         // keep user_id same
        None,                         // keep person_id same
    )
    .await?;

    assert_eq!(updated.contact_type, ContactType::Phone);
    assert_eq!(updated.location.as_deref(), Some("Munich"));
    assert_eq!(updated.user_id, created.user_id);

    // 8. DELETE
    let deleted_id = delete_contact(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    // 9. VERIFY DELETION
    let remaining = get_all_contacts(&pool).await?;
    assert!(!remaining.iter().any(|c| c.id == created.id));

    Ok(())
}
