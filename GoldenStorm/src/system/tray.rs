use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use tray_icon::{TrayIcon, TrayIconBuilder, Icon};
use muda::{Menu, MenuItem};
use crate::system::logging::{self, LogTarget};

pub struct TrayController {
    tray: Arc<Mutex<Option<TrayIcon>>>,
    install_dir: PathBuf,
}

impl TrayController {
    pub fn new(install_dir: &PathBuf) -> Self {
        let tray = Self::create_tray_icon(install_dir);

        Self {
            tray: Arc::new(Mutex::new(Some(tray))),
            install_dir: install_dir.clone(),
        }
    }

    fn load_icon_path(path: PathBuf) -> Option<Icon> {
        Icon::from_path(path, None).ok()
    }

    fn create_tray_icon(install_dir: &PathBuf) -> TrayIcon {
        // Build menu using muda API
        let menu = {
            let mut m = Menu::new();

            let open_item = MenuItem::new("Open GoldenStorm", true, None);
            let pause_item = MenuItem::new("Pause Alerts", true, None);
            let exit_item = MenuItem::new("Exit Agent", true, None);

            m.append(&open_item).unwrap();
            m.append(&pause_item).unwrap();
            m.append(&exit_item).unwrap();

            m
        };

        let icon_path = install_dir.join("assets/icons/app.ico");
        let icon = Self::load_icon_path(icon_path)
            .expect("Failed to load app.ico");

        TrayIconBuilder::new()
            .with_icon(icon)
            .with_tooltip("GoldenStorm Agent")
            .with_menu(Box::new(menu))
            .build()
            .expect("Failed to create tray icon")
    }

    fn load_icon(&self, filename: &str) -> Option<Icon> {
        let path = self.install_dir.join("assets/icons").join(filename);
        Self::load_icon_path(path)
    }

    pub fn set_normal_icon(&self) {
        logging::info(LogTarget::Agent, "Tray: switching to normal icon.");

        let icon = self.load_icon("app.ico");
        if icon.is_none() {
            return;
        }

        let mut tray_lock = self.tray.lock().unwrap();
        if let Some(tray) = tray_lock.as_mut() {
            tray.set_icon(Some(icon.unwrap()));
            tray.set_tooltip(Some("GoldenStorm Agent"));
        }
    }

    pub fn set_alert_icon(&self) {
        logging::warn(LogTarget::Agent, "Tray: switching to alert icon.");

        let icon = self.load_icon("alert.ico");
        if icon.is_none() {
            return;
        }

        let mut tray_lock = self.tray.lock().unwrap();
        if let Some(tray) = tray_lock.as_mut() {
            tray.set_icon(Some(icon.unwrap()));
            tray.set_tooltip(Some("⚠ Severe Weather Alert"));
        }
    }

    pub fn notify_emergency(&self, event: &str) {
        logging::warn(LogTarget::Agent, &format!("Emergency tray notify: {}", event));

        let _ = notify_rust::Notification::new()
            .summary("⚠ EMERGENCY ALERT ⚠")
            .body(event)
            .show();
    }

    pub fn show_notification(&self, title: &str, body: &str) {
        let _ = notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .show();
    }
}
