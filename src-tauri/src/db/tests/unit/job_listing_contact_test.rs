use crate::db::enums::Role;
use crate::db::job_listing_contact::{
    create_job_listing_contact, delete_job_listing_contact, get_all_job_listing_contacts,
    get_job_listing_contact_by_id, get_job_listing_contacts_by_listing_id,
    get_job_listing_contacts_by_person_id, update_job_listing_contact,
};
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_job_listing_contact_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

    let created = create_job_listing_contact(&pool, 1, 1, Some(&Role::HR)).await?;
    assert_eq!(created.role, Some(Role::HR));

    let fetched = get_job_listing_contact_by_id(&pool, created.id).await?;
    assert_eq!(fetched.person_id, 1);

    let all = get_all_job_listing_contacts(&pool).await?;
    assert!(all.len() >= 1);

    let by_listing = get_job_listing_contacts_by_listing_id(&pool, 1).await?;
    assert!(by_listing.len() >= 1);

    let by_person = get_job_listing_contacts_by_person_id(&pool, 1).await?;
    assert!(by_person.len() >= 1);

    let updated = update_job_listing_contact(&pool, created.id, 1, 1, Some(&Role::Manager)).await?;
    assert_eq!(updated.role, Some(Role::Manager));

    let deleted_id = delete_job_listing_contact(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
