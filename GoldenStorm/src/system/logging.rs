use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::system::config::appdata_dir;

const MAX_LOG_FILES: usize = 5;
const MAX_LOG_SIZE_BYTES: u64 = 2 * 1024 * 1024; // 2 MB

#[derive(Clone, Copy)]
pub enum LogTarget {
    Ui,
    Agent,
}

fn log_file_base(target: LogTarget) -> &'static str {
    match target {
        LogTarget::Ui => "goldenstorm_ui.log",
        LogTarget::Agent => "goldenstorm_agent.log",
    }
}

fn log_path(target: LogTarget) -> io::Result<PathBuf> {
    Ok(appdata_dir()?.join(log_file_base(target)))
}

fn rotate_logs(target: LogTarget) -> io::Result<()> {
    let base = log_file_base(target);
    let dir = appdata_dir()?;

    let main = dir.join(base);
    if !main.exists() {
        return Ok(());
    }

    let metadata = fs::metadata(&main)?;
    if metadata.len() < MAX_LOG_SIZE_BYTES {
        return Ok(());
    }

    // Shift old logs: .4 -> .5, .3 -> .4, ..., .1 -> .2, main -> .1
    for i in (1..=MAX_LOG_FILES).rev() {
        let src = if i == 1 {
            dir.join(base)
        } else {
            dir.join(format!("{}.{}", base, i - 1))
        };

        let dst = dir.join(format!("{}.{}", base, i));

        if src.exists() {
            let _ = fs::rename(&src, &dst);
        }
    }

    Ok(())
}

pub fn init_logging(target: LogTarget) -> io::Result<()> {
    crate::system::config::ensure_appdata_dir()?;
    rotate_logs(target)?;

    // Optional: developer-friendly console output
    println!("Logging initialized for {:?}", target);

    Ok(())
}

pub fn log_line(target: LogTarget, level: &str, msg: &str) {
    if let Err(e) = write_log_line(target, level, msg) {
        eprintln!("Logging error: {}", e);
    }
}

fn write_log_line(target: LogTarget, level: &str, msg: &str) -> io::Result<()> {
    let path = log_path(target)?;
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    writeln!(file, "[{}] [{}] {}", timestamp, level, msg)?;
    Ok(())
}

// Convenience helpers
pub fn info(target: LogTarget, msg: &str) {
    log_line(target, "INFO", msg);
}

pub fn warn(target: LogTarget, msg: &str) {
    log_line(target, "WARN", msg);
}

pub fn error(target: LogTarget, msg: &str) {
    log_line(target, "ERROR", msg);
}
