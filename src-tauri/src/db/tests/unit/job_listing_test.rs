use crate::db::models::enums::Role;
use crate::db::queries::job_listing_contact::{
    create_job_listing_contact, delete_job_listing_contact, get_all_job_listing_contacts,
    get_job_listing_contact_by_id, get_job_listing_contacts_by_listing_id,
    get_job_listing_contacts_by_person_id, update_job_listing_contact,
};
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_job_listing_contact_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

    // 1. CREATE
    let created = create_job_listing_contact(&pool, 1, 1, Some(&Role::HR)).await?;
    assert_eq!(created.role, Some(Role::HR));

    // 2. READ BY ID
    let fetched = get_job_listing_contact_by_id(&pool, created.id).await?;
    assert_eq!(fetched.person_id, 1);

    // 3. READ ALL
    let all = get_all_job_listing_contacts(&pool).await?;
    assert_eq!(all.len(), 1);

    // 4. READ BY JOB LISTING
    let by_listing = get_job_listing_contacts_by_listing_id(&pool, 1).await?;
    assert_eq!(by_listing.len(), 1);

    // 5. READ BY PERSON
    let by_person = get_job_listing_contacts_by_person_id(&pool, 1).await?;
    assert_eq!(by_person.len(), 1);

    // 6. UPDATE
    let updated = update_job_listing_contact(&pool, created.id, 1, 1, Some(&Role::Manager)).await?;
    assert_eq!(updated.role, Some(Role::Manager));

    // 7. DELETE
    let deleted_id = delete_job_listing_contact(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
