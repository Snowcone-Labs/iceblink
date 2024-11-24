use iceblink_sync::cli;
use iceblink_sync::ServerOptions;
use std::error::Error;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let settings = cli::get_settings();

    tracing_subscriber::fmt()
        .with_max_level(settings.logging.unwrap_or_else(|| {
            if cfg!(debug_assertions) {
                cli::LoggingLevel::Debug
            } else {
                cli::LoggingLevel::Info
            }
        }))
        .init();

    match &settings.command {
        cli::Commands::Serve {
            port,
            client_id,
            client_secret,
            oauth_server,
            jwt_secret,
            redirect_uri,
            frontfacing,
        } => {
            info!("Iceblink Sync Server");

            iceblink_sync::serve(ServerOptions {
                port: port.unwrap_or(8085),
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                oauth_server: oauth_server
                    .clone()
                    .unwrap_or("https://pfapi.snowflake.blue".to_string()),
                redirect_uri: redirect_uri.to_string(),
                jwt_secret: jwt_secret.to_string(),
                frontfacing: frontfacing
                    .clone()
                    .unwrap_or("http://localhost:8085".to_string()),
            })
            .await;
        }
    }

    Ok(())
}
