use crate::db::models::enums::Role;
use crate::db::queries::person::{
    create_person, delete_person, get_all_persons, get_person_by_id, update_person,
};
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_person_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

    let created = create_person(
        &pool,
        "John",
        "Doe",
        Some("john@example.com"),
        Some("+49 123 456"),
        Some(&Role::Recruiter),
        Some("https://linkedin.com/in/johndoe"),
    )
    .await?;

    assert_eq!(created.first_name, "John");
    assert_eq!(created.role, Some(Role::Recruiter));

    let fetched = get_person_by_id(&pool, created.id).await?;
    assert_eq!(fetched.email.as_deref(), Some("john@example.com"));

    let all = get_all_persons(&pool).await?;
    assert!(all.len() >= 1);

    let updated = update_person(
        &pool,
        created.id,
        "Jane",
        "Doe",
        Some("jane@example.com"),
        Some("+49 321 654"),
        Some(&Role::Manager),
        Some("https://linkedin.com/in/janedoe"),
    )
    .await?;

    assert_eq!(updated.first_name, "Jane");
    assert_eq!(updated.role, Some(Role::Manager));

    let deleted_id = delete_person(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
