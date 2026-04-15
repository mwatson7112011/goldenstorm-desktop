use serde_json::Value;
use wry::webview::WebView;

use crate::ui::state::UiState;
use crate::system::logging::{self, LogTarget};

pub struct IpcHandlers {
    pub ui_state: UiState,
}

impl IpcHandlers {
    pub fn new(ui_state: UiState) -> Self {
        Self { ui_state }
    }

    pub fn handle(&self, window: &WebView, msg: &str) {
        logging::info(LogTarget::Ui, &format!("IPC Received: {}", msg));

        let parsed: Value = match serde_json::from_str(msg) {
            Ok(v) => v,
            Err(_) => {
                logging::error(LogTarget::Ui, "Invalid IPC JSON");
                return;
            }
        };

        let action = parsed["action"].as_str().unwrap_or("");

        match action {
            "set_persona" => {
                if let Some(p) = parsed["value"].as_str() {
                    self.ui_state.set_persona(p);
                    logging::info(LogTarget::Ui, &format!("Persona set to {}", p));
                }
            }

            "toggle_chaos" => {
                let enabled = parsed["value"].as_bool().unwrap_or(false);
                self.ui_state.set_chaos(enabled);
                logging::info(LogTarget::Ui, &format!("Chaos mode: {}", enabled));
            }

            "manual_refresh" => {
                logging::info(LogTarget::Ui, "Manual refresh requested");

                // Tell JS to show a spinner or something
                let _ = window.evaluate_script("window.manualRefreshAck()");
            }

            _ => {
                logging::warn(LogTarget::Ui, &format!("Unknown IPC action: {}", action));
            }
        }
    }
}
