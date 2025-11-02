use crate::db::schema::INIT_SQL;
use crate::logger::*;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{fs, fs::OpenOptions, path::PathBuf};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // 1. Determine persistent path: ~/.JobTrackr/jobtrackr.db
    let home_dir = dirs::home_dir().ok_or_else(|| {
        error!("Failed to locate home directory");
        sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Missing home directory",
        ))
    })?;

    let base_dir = home_dir.join(".JobTrackr");
    if let Err(e) = fs::create_dir_all(&base_dir) {
        error!("Failed to create ~/.JobTrackr directory: {}", e);
        return Err(sqlx::Error::Io(e));
    }

    let db_path: PathBuf = base_dir.join("jobtrackr.db");
    let db_url = format!("sqlite://{}", db_path.display());
    info!("Opening database at {}", db_url);

    // 2. Ensure file exists before opening
    if !db_path.exists() {
        info!("Database file not found — creating empty file.");
        if let Err(e) = OpenOptions::new().create(true).write(true).open(&db_path) {
            error!("Failed to create database file: {}", e);
            return Err(sqlx::Error::Io(e));
        }
    }

    // 3. Connect to SQLite
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;

    // 4. Check schema existence
    let (table_count,): (i64,) =
        match sqlx::query_as("SELECT COUNT(name) FROM sqlite_master WHERE type='table';")
            .fetch_one(&pool)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!("Failed to query sqlite_master: {}", e);
                return Err(e);
            }
        };

    // 5. Initialize schema if empty
    if table_count == 0 {
        info!("Database empty — initializing schema...");
        for stmt in INIT_SQL.split_terminator(';') {
            let trimmed = stmt.trim();
            if !trimmed.is_empty() {
                if let Err(e) = sqlx::query(trimmed).execute(&pool).await {
                    error!("Schema statement failed: {}", e);
                    return Err(e);
                }
            }
        }
        pool.close().await;
        info!("Schema successfully created at {}", db_path.display());
    } else {
        info!("Existing schema detected — skipping initialization.");
    }

    // 6. Verify file exists physically
    if !db_path.exists() {
        error!("Database file was not created — check write permissions");
        return Err(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Database file missing after initialization",
        )));
    }

    // 7. Reconnect after schema initialization
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;

    info!("Database ready for use at {}", db_path.display());
    Ok(pool)
}
