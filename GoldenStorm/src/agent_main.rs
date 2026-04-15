//! GoldenStormAgent.exe
//! Runs the background weather watcher + tray icon.

mod system;
mod backend;

use system::background_agent::BackgroundAgent;
use system::logging::{self, LogTarget};

#[tokio::main]
async fn main() {
    // Initialize logging
    logging::init_logging(LogTarget::Agent).unwrap();
    logging::info(LogTarget::Agent, "GoldenStormAgent.exe launched.");

    // Create the agent (loads config, initializes tray, etc.)
    let mut agent = BackgroundAgent::new().await;

    // Run the background loop forever
    agent.run().await;
}
