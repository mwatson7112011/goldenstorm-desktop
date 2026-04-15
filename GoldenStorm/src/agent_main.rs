//! GoldenStormAgent.exe
//! Runs the background weather watcher + tray icon.

// Declare module folders so Rust can see them
mod backend;
mod system;
mod ui;
mod utils;
mod weather;

use crate::system::background_agent::BackgroundAgent;
use crate::system::logging::{self, LogTarget};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize logging
    logging::init_logging(LogTarget::Agent).unwrap();
    logging::info(LogTarget::Agent, "GoldenStormAgent.exe launched.");

    // Create the agent (loads config, initializes tray, etc.)
    let mut agent: BackgroundAgent = BackgroundAgent::new().await;

    // Run the background loop forever (includes flashing + polling)
    agent.run().await;
}
