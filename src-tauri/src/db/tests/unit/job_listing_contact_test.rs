use crate::db::models::enums::Role;
use crate::db::queries::job_listing_contact::{
    create_job_listing_contact, delete_job_listing_contact, get_all_job_listing_contacts,
    get_job_listing_contact_by_id, get_job_listing_contacts_by_listing_id,
    get_job_listing_contacts_by_person_id, update_job_listing_contact,
};
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_job_listing_contact_crud_flow() -> Result<(), sqlx::Error> {
    // 1. Setup isolated DB
    let pool = setup_test_db().await;

    // 2. CREATE
    let created = create_job_listing_contact(&pool, 1, 1, Some(&Role::HR)).await?;
    assert_eq!(created.role, Some(Role::HR));
    assert_eq!(created.job_listing_id, 1);
    assert_eq!(created.person_id, 1);

    // 3. READ BY ID
    let fetched = get_job_listing_contact_by_id(&pool, created.id).await?;
    assert_eq!(fetched.person_id, 1);
    assert_eq!(fetched.role, Some(Role::HR));

    // 4. READ ALL
    let all = get_all_job_listing_contacts(&pool).await?;
    assert!(all.iter().any(|c| c.id == created.id));

    // 5. READ BY JOB LISTING
    let by_listing = get_job_listing_contacts_by_listing_id(&pool, 1).await?;
    assert!(by_listing.iter().any(|c| c.id == created.id));

    // 6. READ BY PERSON
    let by_person = get_job_listing_contacts_by_person_id(&pool, 1).await?;
    assert!(by_person.iter().any(|c| c.id == created.id));

    // 7. UPDATE (using new Option-based signature)
    let updated = update_job_listing_contact(
        &pool,
        created.id,
        Some(1),              // job_listing_id stays same
        Some(1),              // person_id stays same
        Some(&Role::Manager), // role changed
    )
    .await?;

    assert_eq!(updated.role, Some(Role::Manager));
    assert_eq!(updated.person_id, 1);
    assert_eq!(updated.job_listing_id, 1);

    // 8. DELETE
    let deleted_id = delete_job_listing_contact(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    // 9. VERIFY DELETION
    let remaining = get_all_job_listing_contacts(&pool).await?;
    assert!(!remaining.iter().any(|c| c.id == created.id));

    Ok(())
}
