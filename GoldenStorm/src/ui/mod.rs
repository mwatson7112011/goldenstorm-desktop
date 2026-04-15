pub mod state;
pub mod ipc_handlers;

use ipc_handlers::IpcHandlers;
use state::UiState;

use wry::webview::WebView;

pub fn build_ipc(ui_state: UiState) -> impl Fn(&WebView, String) {
    let handlers = IpcHandlers::new(ui_state);

    move |window: &WebView, msg: String| {
        handlers.handle(window, &msg);
    }
}

