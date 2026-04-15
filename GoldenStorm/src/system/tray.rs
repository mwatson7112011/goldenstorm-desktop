use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tray_icon::{
    TrayIcon, TrayIconBuilder, MenuBuilder, MenuItemBuilder, Icon, MessageType,
};

use crate::system::logging::{self, LogTarget};

pub struct TrayController {
    tray: Arc<Mutex<Option<TrayIcon>>>,
    install_dir: PathBuf,
    flashing: Arc<Mutex<bool>>,
}

impl TrayController {
    pub fn new(install_dir: &PathBuf) -> Self {
        let tray = Self::create_tray_icon(install_dir);

        let controller = Self {
            tray: Arc::new(Mutex::new(Some(tray))),
            install_dir: install_dir.clone(),
            flashing: Arc::new(Mutex::new(false)),
        };

        controller
    }

    fn create_tray_icon(install_dir: &PathBuf) -> TrayIcon {
        let menu = MenuBuilder::new()
            .item(&MenuItemBuilder::new("Open GoldenStorm").build())
            .separator()
            .item(&MenuItemBuilder::new("Pause Alerts").build())
            .item(&MenuItemBuilder::new("Exit Agent").build())
            .build();

        let icon_path = install_dir.join("assets").join("icons").join("app.ico");
        let icon = Icon::from_file(icon_path.to_string_lossy().as_ref(), None)
            .expect("Failed to load app.ico");

        TrayIconBuilder::new()
            .with_tooltip("GoldenStorm Agent")
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .build()
            .expect("Failed to create tray icon")
    }

    fn load_icon(&self, filename: &str) -> Option<Icon> {
        let path = self.install_dir.join("assets").join("icons").join(filename);
        match Icon::from_file(path.to_string_lossy().as_ref(), None) {
            Ok(icon) => Some(icon),
            Err(e) => {
                logging::error(
                    LogTarget::Agent,
                    &format!("Failed to load icon {:?}: {}", path, e),
                );
                None
            }
        }
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
            let normal_icon_path = install_dir.join("assets/icons/app.ico");
            let alert_icon_path = install_dir.join("assets/icons/alert.ico");

            let normal_icon = Icon::from_file(normal_icon_path.to_string_lossy().as_ref(), None).ok();
            let alert_icon = Icon::from_file(alert_icon_path.to_string_lossy().as_ref(), None).ok();

            if normal_icon.is_none() || alert_icon.is_none() {
                return;
            }

            let mut toggle = false;

            while *flashing_flag.lock().unwrap() {
                {
                    let mut tray_lock = tray_ref.lock().unwrap();
                    if let Some(tray) = tray_lock.as_mut() {
                        if toggle {
                            tray.set_icon(alert_icon.clone().unwrap());
                        } else {
                            tray.set_icon(normal_icon.clone().unwrap());
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

    pub fn show_notification(&self, title: &str, message: &str) {
        logging::warn(
            LogTarget::Agent,
            &format!("Tray notification: {} - {}", title, message),
        );

        let mut tray_lock = self.tray.lock().unwrap();
        if let Some(tray) = tray_lock.as_mut() {
            tray.show_message(title, message, MessageType::Info);
        }
    }

    pub fn notify_emergency(&self, event: &str) {
        logging::warn(LogTarget::Agent, &format!("Emergency tray notify: {}", event));

        let mut tray_lock = self.tray.lock().unwrap();
        if let Some(tray) = tray_lock.as_mut() {
            tray.show_message(
                "⚠ EMERGENCY ALERT ⚠",
                event,
                MessageType::Error,
            );
        }
    }
}
