pub mod auth;
pub mod cli;
pub mod models;
pub mod routes;
pub mod server;
pub mod utils;

use cli::Commands;
use server::ServerOptions;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let settings = cli::get_settings();

    tracing_subscriber::fmt()
        .with_max_level(settings.logging.unwrap_or(cli::LoggingLevel::Info))
        .init();

    match &settings.command {
        Commands::Serve {
            port,
            client_id,
            client_secret,
            oauth_server,
            jwt_secret,
        } => {
            info!("Iceblink Sync Server");

            server::create_server(ServerOptions {
                port: port.unwrap_or(8085),
                oauth: server::OAuthOptions {
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    config: auth::OpenId::get(
                        oauth_server
                            .clone()
                            .unwrap_or("https://pfapi.snowflake.blue".to_string()),
                    )
                    .await
                    .expect("Unable to gather OpenID Connect Discovery configuration"),
                },
                jwt_secret: jwt_secret.to_string(),
            })
            .await;
        }
    }

    Ok(())
}
