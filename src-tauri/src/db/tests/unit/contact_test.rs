use crate::db::contact::{
    create_contact, delete_contact, get_all_contacts, get_contact_by_id, get_contacts_by_person_id,
    get_contacts_by_user_id, update_contact,
};
use crate::db::enums::ContactType;
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_contact_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

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

    let fetched = get_contact_by_id(&pool, created.id).await?;
    assert_eq!(fetched.location.as_deref(), Some("Berlin"));

    let all = get_all_contacts(&pool).await?;
    assert!(all.len() >= 1);

    let by_user = get_contacts_by_user_id(&pool, 1).await?;
    assert!(by_user.len() >= 1);

    let by_person = get_contacts_by_person_id(&pool, 1).await?;
    assert!(by_person.len() >= 1);

    let updated = update_contact(
        &pool,
        created.id,
        &ContactType::Phone,
        "2025-02-02T00:00:00Z",
        Some("Munich"),
        1,
        Some(1),
    )
    .await?;

    assert_eq!(updated.contact_type, ContactType::Phone);
    assert_eq!(updated.location.as_deref(), Some("Munich"));

    let deleted_id = delete_contact(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
