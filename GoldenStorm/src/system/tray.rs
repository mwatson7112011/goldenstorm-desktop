use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tray_icon::{TrayIcon, TrayIconBuilder, Icon};
use muda::{Menu, MenuItem, Submenu};

use crate::system::logging::{self, LogTarget};

pub struct TrayController {
    tray: Arc<Mutex<Option<TrayIcon>>>,
    install_dir: PathBuf,
    flashing: Arc<Mutex<bool>>,
}

impl TrayController {
    pub fn new(install_dir: &PathBuf) -> Self {
        let tray = Self::create_tray_icon(install_dir);

        Self {
            tray: Arc::new(Mutex::new(Some(tray))),
            install_dir: install_dir.clone(),
            flashing: Arc::new(Mutex::new(false)),
        }
    }

    fn load_icon_path(path: PathBuf) -> Option<Icon> {
        Icon::from_path(path, None).ok()
    }

    fn create_tray_icon(install_dir: &PathBuf) -> TrayIcon {
        // Build menu using muda API
        let mut menu = Menu::new();

        let open_item = MenuItem::new("Open GoldenStorm", true, None);
        let pause_item = MenuItem::new("Pause Alerts", true, None);
        let exit_item = MenuItem::new("Exit Agent", true, None);

        menu.append(&open_item).unwrap();
        menu.append(&pause_item).unwrap();
        menu.append(&exit_item).unwrap();

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
            tray.set_icon(icon.unwrap());
            tray.set_tooltip("GoldenStorm Agent");
        }

        self.stop_flashing();
    }

    pub fn set_alert_icon(&self, flash: bool) {
        logging::warn(LogTarget::Agent, "Tray: switching to alert icon.");

        let icon = self.load_icon("alert.ico");
        if icon.is_none() {
            return;
        }

        let mut tray_lock = self.tray.lock().unwrap();
        if let Some(tray) = tray_lock.as_mut() {
            tray.set_icon(icon.unwrap());
            tray.set_tooltip("⚠ Severe Weather Alert");
        }

        if flash {
            self.start_flashing();
        }
    }

    fn start_flashing(&self) {
        let flashing_flag = self.flashing.clone();
        let tray_ref = self.tray.clone();
        let install_dir = self.install_dir.clone();

        {
            let mut f = flashing_flag.lock().unwrap();
            *f = true;
        }

        thread::spawn(move || {
            let normal_icon = Icon::from_path(
                install_dir.join("assets/icons/app.ico"),
                None,
            ).ok();

            let alert_icon = Icon::from_path(
                install_dir.join("assets/icons/alert.ico"),
                None,
            ).ok();

            if normal_icon.is_none() || alert_icon.is_none() {
                return;
            }

            let normal_icon = normal_icon.unwrap();
            let alert_icon = alert_icon.unwrap();

            let mut toggle = false;

            while *flashing_flag.lock().unwrap() {
                {
                    let mut tray_lock = tray_ref.lock().unwrap();
                    if let Some(tray) = tray_lock.as_mut() {
                        if toggle {
                            tray.set_icon(alert_icon.clone());
                        } else {
                            tray.set_icon(normal_icon.clone());
                        }
                    }
                }

                toggle = !toggle;
                thread::sleep(Duration::from_millis(600));
            }
        });
    }

    pub fn stop_flashing(&self) {
        let mut f = self.flashing.lock().unwrap();
        *f = false;
    }

    pub fn notify_emergency(&self, event: &str) {
        logging::warn(LogTarget::Agent, &format!("Emergency tray notify: {}", event));

        // Use notify-rust for system notifications
        let _ = notify_rust::Notification::new()
            .summary("⚠ EMERGENCY ALERT ⚠")
            .body(event)
            .show();
    }
}
