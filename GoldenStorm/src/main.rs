//! GoldenStorm.exe (UI Application)
//! Loads the Wry/Tao window, polls JSON state from the agent,
//! and enters emergency mode when launched with --tornado-alert.

// Declare module folders so Rust can see them
mod backend;
mod system;
mod ui;
mod utils;
mod weather;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use tao::platform::windows::IconExtWindows;
use tao::window::{WindowBuilder, Icon};

use wry::WebViewBuilder;

// Messages sent from background thread → main thread
#[derive(Debug, Clone)]
enum UiMessage {
    WeatherUpdate(String),
    AlertUpdate(String),
}

fn main() {
    // Detect emergency launch mode
    let args: Vec<String> = env::args().collect();
    let tornado_alert_launch = args.iter().any(|a| a == "--tornado-alert");

    if tornado_alert_launch {
        println!("🚨 GoldenStorm launched due to Tornado Warning — entering emergency UI mode.");
    } else {
        println!("GoldenStorm UI launched normally.");
    }

    // Build event loop with proxy support
    let event_loop: EventLoop<UiMessage> = EventLoop::with_user_event();
    let proxy: EventLoopProxy<UiMessage> = event_loop.create_proxy();

    // Load window icon
    let window_icon = load_icon("app.ico")
        .expect("Failed to load app.ico for window icon");

    // Create window
    let window = WindowBuilder::new()
        .with_title("GoldenStorm Weather")
        .with_inner_size(LogicalSize::new(420.0, 720.0))
        .with_resizable(true)
        .with_window_icon(Some(window_icon))
        .build(&event_loop)
        .expect("Failed to create window");

    // Load index.html
    let base = install_base_dir();
    let index_path = base.join("assets").join("index.html");
    let index_url = format!("file:///{}", index_path.to_string_lossy());

    let webview = WebViewBuilder::new(&window)
        .with_url(&index_url)
        .build()
        .expect("Failed to build WebView");

    // If launched due to tornado alert, notify UI
    if tornado_alert_launch {
        let _ = webview.evaluate_script(
            "window.dispatchEvent(new CustomEvent('tornadoAlertLaunch'));"
        );
    }

    // Spawn background polling thread (2-second interval)
    {
        let proxy_clone = proxy.clone();
        std::thread::spawn(move || {
            let weather_path = base.join("assets/state/latest_weather.json");
            let alert_path = base.join("assets/state/latest_alert.json");

            loop {
                // Weather JSON
                if let Ok(json) = fs::read_to_string(&weather_path) {
                    let _ = proxy_clone.send_event(UiMessage::WeatherUpdate(json));
                }

                // Alert JSON
                if let Ok(json) = fs::read_to_string(&alert_path) {
                    let _ = proxy_clone.send_event(UiMessage::AlertUpdate(json));
                }

                std::thread::sleep(Duration::from_millis(2000)); // 2-second polling
            }
        });
    }

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // Handle messages from background thread
            Event::UserEvent(msg) => {
                match msg {
                    UiMessage::WeatherUpdate(json) => {
                        let js = format!(
                            "window.dispatchEvent(new CustomEvent('weatherUpdate', {{ detail: {} }}));",
                            json
                        );
                        let _ = webview.evaluate_script(&js);
                    }
                    UiMessage::AlertUpdate(json) => {
                        let js = format!(
                            "window.dispatchEvent(new CustomEvent('alertUpdate', {{ detail: {} }}));",
                            json
                        );
                        let _ = webview.evaluate_script(&js);
                    }
                }
            }

            // Window close
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            _ => {}
        }
    });
}

/// Loads an ICO file from the installed directory
fn load_icon(filename: &str) -> Option<Icon> {
    let path = install_base_dir()
        .join("assets")
        .join("icons")
        .join(filename);

    Icon::from_path(path, None).ok()
}

/// Installed directory: C:\Program Files\GoldenStorm\
fn install_base_dir() -> PathBuf {
    PathBuf::from(r"C:\Program Files\GoldenStorm")
}
