pub mod cli;
pub mod routes;
pub mod server;
use cli::Commands;
use server::ServerOptions;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = cli::get_settings();

    tracing_subscriber::fmt()
        .with_max_level(settings.logging.unwrap_or(cli::LoggingLevel::Info))
        .init();

    match &settings.command {
        Commands::Serve { port } => {
            info!("Iceblink Sync Server");
            server::create_server(ServerOptions {
                port: port.unwrap_or(8085),
            })
            .await;
        }
    }

    Ok(())
}
