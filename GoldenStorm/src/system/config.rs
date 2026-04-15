use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub lat: f64,
    pub lon: f64,
    pub city: String,
    pub persona: String,
    pub chaos: bool,
    pub refresh_interval_secs: u64,
    pub sound_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            lat: 0.0,
            lon: 0.0,
            city: "Unknown".to_string(),
            persona: "Serious".to_string(),
            chaos: false,
            refresh_interval_secs: 60,
            sound_enabled: true,
        }
    }
}

pub fn appdata_dir() -> io::Result<PathBuf> {
    let base = dirs::config_dir()
        .or_else(|| dirs::data_dir())
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No config dir found"))?;

    Ok(base.join("GoldenStorm"))
}

pub fn config_path() -> io::Result<PathBuf> {
    Ok(appdata_dir()?.join("config.json"))
}

pub fn ensure_appdata_dir() -> io::Result<()> {
    let dir = appdata_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(())
}

pub fn load_config() -> io::Result<AppConfig> {
    ensure_appdata_dir()?;
    let path = config_path()?;

    if !path.exists() {
        let default_cfg = AppConfig::default();
        save_config(&default_cfg)?;
        return Ok(default_cfg);
    }

    let data = fs::read_to_string(&path)?;
    let cfg: AppConfig = serde_json::from_str(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(cfg)
}

pub fn save_config(cfg: &AppConfig) -> io::Result<()> {
    ensure_appdata_dir()?;
    let path = config_path()?;
    let json = serde_json::to_string_pretty(cfg)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let mut file = fs::File::create(&path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
