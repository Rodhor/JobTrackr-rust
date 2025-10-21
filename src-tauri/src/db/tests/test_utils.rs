use sqlx::{Executor, SqlitePool};

pub async fn setup_test_db() -> SqlitePool {
    // Create isolated in-memory SQLite database
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    // Load schema
    let schema =
        std::fs::read_to_string("migrations/0001_init.sql").expect("Failed to read schema file");
    pool.execute(schema.as_str()).await.unwrap();

    // Seed minimal data for foreign key dependencies
    pool.execute(
        r#"
        INSERT INTO user (first_name, last_name, email)
        VALUES ('Test', 'User', 'test@example.com');

        INSERT INTO company (name)
        VALUES ('Default Company');

        INSERT INTO job_listing (company_id, title)
        VALUES (1, 'Default Job');

        INSERT INTO person (first_name, last_name)
        VALUES ('John', 'Doe');

        INSERT INTO contact (
            contact_type, contact_date, user_id, person_id
        ) VALUES ('email', CURRENT_TIMESTAMP, 1, 1);
        "#,
    )
    .await
    .unwrap();

    pool
}
