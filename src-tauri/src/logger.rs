// src/logger.rs
use chrono::Local;
use std::{fs, fs::OpenOptions, sync::Mutex};
use tracing_subscriber::fmt;

pub fn init() {
    let home = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("FATAL: cannot determine home directory");
        std::process::exit(1);
    });

    let month = Local::now().format("%Y-%m").to_string();
    let dir = home.join(".JobTrackr").join("logs").join(month);
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("FATAL: cannot create log directory {:?}: {}", dir, e);
        std::process::exit(1);
    }

    let file_path = dir.join(format!("jobtrackr.{}.log", Local::now().format("%Y-%m-%d")));
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .unwrap_or_else(|e| {
            eprintln!("FATAL: cannot open log file {:?}: {}", file_path, e);
            std::process::exit(1);
        });

    let writer = Mutex::new(file);
    let timer = fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".into());

    let subscriber = fmt()
        .with_writer(writer)
        .with_timer(timer)
        .with_target(false)
        .with_ansi(false)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("set global subscriber failed");
    tracing::info!("Logger initialized at {:?}", file_path);
}

pub use tracing::{debug, error, info, warn};
