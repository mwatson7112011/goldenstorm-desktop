//! src/system/background_agent.rs
//! Refactored async BackgroundAgent for GoldenStormAgent.exe

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::time::{sleep, Duration};

use crate::backend::weather_api::WeatherApiClient;
use crate::backend::weather_models::{ApiWeatherState, ApiAlert};
use crate::system::logging::{self, LogTarget};
use crate::system::tray::TrayController;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    None,
    Watch,
    Advisory,
    Warning,
    TornadoWarning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertState {
    pub severity: AlertSeverity,
    pub headline: String,
    pub description: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherState {
    pub temperature: f32,
    pub condition: String,
    pub last_updated: String,
}

pub struct BackgroundAgent {
    client: WeatherApiClient,
    tray: TrayController,
    install_dir: PathBuf,
    state_dir: PathBuf,
    weather_path: PathBuf,
    alert_path: PathBuf,
    last_alert: AlertState,
    poll_interval: Duration,
}

impl BackgroundAgent {
    pub async fn new() -> Self {
        let install_dir = PathBuf::from(r"C:\Program Files\GoldenStorm");
        let state_dir = install_dir.join("assets").join("state");

        if let Err(e) = fs::create_dir_all(&state_dir).await {
            logging::error(
                LogTarget::Agent,
                &format!("Failed to create state dir {:?}: {}", state_dir, e),
            );
        }

        let weather_path = state_dir.join("latest_weather.json");
        let alert_path = state_dir.join("latest_alert.json");

        let tray = TrayController::new(&install_dir);
        tray.set_normal_icon();

        let last_alert = AlertState {
            severity: AlertSeverity::None,
            headline: "No active alerts".into(),
            description: String::new(),
            expires_at: None,
        };

        BackgroundAgent {
            client: WeatherApiClient::new(),
            tray,
            install_dir,
            state_dir,
            weather_path,
            alert_path,
            last_alert,
            poll_interval: Duration::from_secs(30),
        }
    }

    pub async fn run(&mut self) {
        logging::info(LogTarget::Agent, "BackgroundAgent run loop started.");

        loop {
            if let Err(e) = self.poll_once().await {
                logging::error(
                    LogTarget::Agent,
                    &format!("Error during poll_once: {}", e),
                );
            }

            sleep(self.poll_interval).await;
        }
    }

    async fn poll_once(&mut self) -> Result<(), String> {
        logging::info(LogTarget::Agent, "Polling weather + alerts…");

        let api_weather = self
            .client
            .fetch_weather()
            .await
            .map_err(|e| format!("fetch_weather failed: {}", e))?;

        let api_alerts = self
            .client
            .fetch_alerts()
            .await
            .map_err(|e| format!("fetch_alerts failed: {}", e))?;

        let weather_state = self.map_weather(api_weather);
        let alert_state = self.map_alerts(api_alerts);

        self.write_json_state(&weather_state, &alert_state).await?;

        // FIX: clone previous alert to avoid borrow conflict
        let prev_alert = self.last_alert.clone();

        self.handle_alert_transition(&prev_alert, &alert_state)
            .await;

        self.last_alert = alert_state;

        Ok(())
    }

    fn map_weather(&self, api: ApiWeatherState) -> WeatherState {
        WeatherState {
            temperature: api.temperature,
            condition: api.condition,
            last_updated: api.last_updated,
        }
    }

    fn map_alerts(&self, api_alerts: Vec<ApiAlert>) -> AlertState {
        if api_alerts.is_empty() {
            return AlertState {
                severity: AlertSeverity::None,
                headline: "No active alerts".into(),
                description: String::new(),
                expires_at: None,
            };
        }

        let mut highest = AlertSeverity::None;
        let mut chosen: Option<&ApiAlert> = None;

        for alert in &api_alerts {
            let sev = self.map_api_severity(&alert.severity);
            if self.is_more_severe(sev, highest) {
                highest = sev;
                chosen = Some(alert);
            }
        }

        if let Some(alert) = chosen {
            AlertState {
                severity: highest,
                headline: alert.headline.clone(),
                description: alert.description.clone(),
                expires_at: alert.expires_at.clone(),
            }
        } else {
            AlertState {
                severity: AlertSeverity::None,
                headline: "No active alerts".into(),
                description: String::new(),
                expires_at: None,
            }
        }
    }

    fn map_api_severity(&self, api_sev: &str) -> AlertSeverity {
        let s = api_sev.to_lowercase();
        if s.contains("tornado warning") {
            AlertSeverity::TornadoWarning
        } else if s.contains("warning") {
            AlertSeverity::Warning
        } else if s.contains("watch") {
            AlertSeverity::Watch
        } else if s.contains("advisory") {
            AlertSeverity::Advisory
        } else {
            AlertSeverity::None
        }
    }

    fn is_more_severe(&self, a: AlertSeverity, b: AlertSeverity) -> bool {
        self.severity_rank(a) > self.severity_rank(b)
    }

    fn severity_rank(&self, s: AlertSeverity) -> u8 {
        match s {
            AlertSeverity::None => 0,
            AlertSeverity::Advisory => 1,
            AlertSeverity::Watch => 2,
            AlertSeverity::Warning => 3,
            AlertSeverity::TornadoWarning => 4,
        }
    }

    async fn write_json_state(
        &self,
        weather: &WeatherState,
        alert: &AlertState,
    ) -> Result<(), String> {
        let weather_json = serde_json::to_string(weather)
            .map_err(|e| format!("serialize weather failed: {}", e))?;
        let alert_json = serde_json::to_string(alert)
            .map_err(|e| format!("serialize alert failed: {}", e))?;

        fs::write(&self.weather_path, weather_json)
            .await
            .map_err(|e| format!("write weather json failed: {}", e))?;

        fs::write(&self.alert_path, alert_json)
            .await
            .map_err(|e| format!("write alert json failed: {}", e))?;

        Ok(())
    }

    async fn handle_alert_transition(
        &mut self,
        prev: &AlertState,
        current: &AlertState,
    ) {
        if prev.severity == AlertSeverity::None && current.severity != AlertSeverity::None {
            match current.severity {
                AlertSeverity::TornadoWarning => {
                    logging::warn(
                        LogTarget::Agent,
                        &format!("New Tornado Warning: {}", current.headline),
                    );
                    self.tray.set_alert_icon(true);
                    self.tray.show_notification(
                        "🚨 Tornado Warning",
                        &current.headline,
                    );
                    self.launch_ui_for_tornado().await;
                }
                AlertSeverity::Warning => {
                    logging::warn(
                        LogTarget::Agent,
                        &format!("New Severe Weather Warning: {}", current.headline),
                    );
                    self.tray.set_alert_icon(true);
                    self.tray.show_notification(
                        "⚠ Severe Weather Warning",
                        &current.headline,
                    );
                }
                _ => {
                    logging::info(
                        LogTarget::Agent,
                        &format!("New alert: {}", current.headline),
                    );
                    self.tray.set_alert_icon(false);
                }
            }
        }

        if prev.severity != AlertSeverity::None && current.severity == AlertSeverity::None {
            logging::info(LogTarget::Agent, "Alerts cleared – returning to normal icon.");
            self.tray.set_normal_icon();
            self.tray.stop_flashing();
        }
    }

    async fn launch_ui_for_tornado(&self) {
        let ui_path = self.install_dir.join("GoldenStorm.exe");

        if !Path::new(&ui_path).exists() {
            logging::error(
                LogTarget::Agent,
                &format!(
                    "GoldenStorm.exe not found at {:?} – cannot auto-open UI for Tornado Warning.",
                    ui_path
                ),
            );
            return;
        }

        logging::info(
            LogTarget::Agent,
            &format!("Launching UI for Tornado Warning: {:?}", ui_path),
        );

        let result = Command::new(&ui_path)
            .arg("--tornado-alert")
            .spawn();

        if let Err(e) = result {
            logging::error(
                LogTarget::Agent,
                &format!("Failed to launch GoldenStorm UI for Tornado Warning: {}", e),
            );
        }
    }
}
