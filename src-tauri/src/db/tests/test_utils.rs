use sqlx::{Executor, SqlitePool};

pub async fn setup_test_db() -> SqlitePool {
    // Create isolated in-memory SQLite database
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    // Load schema from migration
    let schema =
        std::fs::read_to_string("migrations/0001_init.sql").expect("Failed to read schema file");
    pool.execute(schema.as_str()).await.unwrap();

    // ======================================================
    // Seed minimal valid data for relational dependencies
    // ======================================================

    // 1. Company (required by job_listing)
    pool.execute(
        r#"
        INSERT INTO company (name, city, country)
        VALUES ('Default Company', 'Berlin', 'Germany');
        "#,
    )
    .await
    .unwrap();

    // 2. Job listing (required by application)
    pool.execute(
        r#"
        INSERT INTO job_listing (company_id, title, work_type)
        VALUES (1, 'Default Job', 'remote');
        "#,
    )
    .await
    .unwrap();

    // 3. Application (required by contact / reminder)
    pool.execute(
        r#"
        INSERT INTO application (job_listing_id, stage, applied_date)
        VALUES (1, 'applied', DATE('now'));
        "#,
    )
    .await
    .unwrap();

    // 4. Person (used by contact and notes)
    pool.execute(
        r#"
        INSERT INTO person (first_name, last_name, email)
        VALUES ('John', 'Doe', 'john@example.com');
        "#,
    )
    .await
    .unwrap();

    // 5. Contact (linked to person + application)
    pool.execute(
        r#"
        INSERT INTO contact (contact_type, contact_date, person_id, application_id, location)
        VALUES ('email', DATE('now'), 1, 1, 'Berlin');
        "#,
    )
    .await
    .unwrap();

    // 6. Note (attached to contact)
    pool.execute(
        r#"
        INSERT INTO note (contact_id, note_type, content)
        VALUES (1, 'general', 'Initial contact note for testing.');
        "#,
    )
    .await
    .unwrap();

    // 7. Reminder (attached to application)
    pool.execute(
        r#"
        INSERT INTO reminder (
            application_id, reminder_date, title, message, is_completed
        ) VALUES (1, DATE('now', '+1 day'), 'Follow-up', 'Send follow-up email', 0);
        "#,
    )
    .await
    .unwrap();

    pool
}
