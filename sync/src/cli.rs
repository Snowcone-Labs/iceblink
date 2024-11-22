use clap::{Parser, Subcommand};
use tracing::level_filters::LevelFilter;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    None,
}

impl From<LoggingLevel> for LevelFilter {
    fn from(value: LoggingLevel) -> Self {
        match value {
            LoggingLevel::Trace => LevelFilter::TRACE,
            LoggingLevel::Debug => LevelFilter::DEBUG,
            LoggingLevel::Info => LevelFilter::INFO,
            LoggingLevel::Warn => LevelFilter::WARN,
            LoggingLevel::Error => LevelFilter::ERROR,
            LoggingLevel::None => LevelFilter::OFF,
        }
    }
}

#[derive(Parser)]
#[command(version, about, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Optional logging level to use. Default is info.
    #[arg(short, long, env = "ICEBLINK_LOGGING_LEVEL")]
    pub logging: Option<LoggingLevel>,
}

#[derive(Subcommand)]
pub enum Commands {
    Serve {
        /// Optional port to use. Default is 8085.
        #[arg(short, long, env = "ICEBLINK_PORT")]
        port: Option<u32>,

        /// JWT secret signing key.
        #[arg(long, env = "ICEBLINK_JWT_SECRET")]
        jwt_secret: String,

        /// OAuth client id.
        #[arg(long, env = "ICEBLINK_OAUTH_CLIENT_ID")]
        client_id: String,

        /// OAuth client secret.
        #[arg(long, env = "ICEBLINK_OAUTH_CLIENT_SECRET")]
        client_secret: String,

        /// Redirect URI for OAuth.
        /// Example: https://iceblink.snowflake.blue/v1/oauth.
        #[arg(long, env = "ICEBLINK_OAUTH_REDIRECT_URI")]
        redirect_uri: String,

        /// OAuth server with OIDC located at /.well-known/openid-configuration.
        /// Do not include a trailing slash.
        /// Defaults to https://pfapi.snowflake.blue.
        #[arg(long, env = "ICEBLINK_OAUTH_SERVER")]
        oauth_server: Option<String>,

        /// URL of itself after passing through a possible reverse proxy.
        /// Should not have a trailing slash.
        /// Used for CORS.
        /// Defaults to http://localhost:8085.
        #[arg(long, env = "ICEBLINK_URL")]
        frontfacing: Option<String>,
    },
}

pub fn get_settings() -> Cli {
    Cli::parse()
}
