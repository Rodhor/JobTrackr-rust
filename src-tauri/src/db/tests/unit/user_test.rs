use crate::db::queries::user::{
    create_user, delete_user, get_all_users, get_user_by_id, update_user,
};
use crate::db::tests::test_utils::setup_test_db;

#[tokio::test]
async fn test_user_crud_flow() -> Result<(), sqlx::Error> {
    let pool = setup_test_db().await;

    // 1. CREATE
    let created = create_user(
        &pool,
        "Alice",
        "Tester",
        "alice@example.com",
        Some("555-1234"),
        Some("Main St 1"),
        Some("12345"),
        Some("Berlin"),
        Some("Germany"),
    )
    .await?;

    assert_eq!(created.first_name, "Alice");
    assert_eq!(created.email, "alice@example.com");

    // 2. READ BY ID
    let fetched = get_user_by_id(&pool, created.id).await?;
    assert_eq!(fetched.email, "alice@example.com");

    // 3. READ ALL
    let users = get_all_users(&pool).await?;
    assert!(users.len() >= 2); // includes seeded 'Test User'

    // 4. UPDATE
    let updated = update_user(
        &pool,
        created.id,
        Some("Alice"),
        Some("Updated"),
        Some("alice.new@example.com"),
        Some("555-9999"),
        Some("Second St 5"),
        Some("54321"),
        Some("Munich"),
        Some("Germany"),
    )
    .await?;

    assert_eq!(updated.last_name, "Updated");
    assert_eq!(updated.city.as_deref(), Some("Munich"));

    // 5. DELETE
    let deleted_id = delete_user(&pool, created.id).await?;
    assert_eq!(deleted_id, created.id);

    Ok(())
}
