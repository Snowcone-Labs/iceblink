pub mod cli;
pub mod server;
use server::ServerOptions;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = cli::get_settings();

    tracing_subscriber::fmt()
        .with_max_level(settings.logging.unwrap_or(cli::LoggingLevel::Info))
        .init();

    info!("Iceblink Sync");

    server::create_server(ServerOptions {
        port: settings.port.unwrap_or(8085),
    })
    .await;

    Ok(())
}
