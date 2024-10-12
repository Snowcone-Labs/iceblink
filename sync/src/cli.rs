use clap::{Parser, Subcommand};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    None,
}

impl From<LoggingLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(value: LoggingLevel) -> Self {
        match value {
            LoggingLevel::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
            LoggingLevel::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            LoggingLevel::Info => tracing_subscriber::filter::LevelFilter::INFO,
            LoggingLevel::Warn => tracing_subscriber::filter::LevelFilter::WARN,
            LoggingLevel::Error => tracing_subscriber::filter::LevelFilter::ERROR,
            LoggingLevel::None => tracing_subscriber::filter::LevelFilter::OFF,
        }
    }
}

#[derive(Parser)]
#[command(version, about, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Optional logging level to use. Default is info
    #[arg(short, long, env = "ICEBLINK_LOGGING_LEVEL")]
    pub logging: Option<LoggingLevel>,
}

#[derive(Subcommand)]
pub enum Commands {
    Serve {
        /// Optional port to use. Default is 8085
        #[arg(short, long, env = "ICEBLINK_PORT")]
        port: Option<u32>,

        /// JWT secret signing key
        #[arg(long, env = "ICEBLINK_JWT_SECRET")]
        jwt_secret: String,

        /// OAuth client id
        #[arg(long, env = "ICEBLINK_OAUTH_CLIENT_ID")]
        client_id: String,

        /// OAuth client secret
        #[arg(long, env = "ICEBLINK_OAUTH_CLIENT_SECRET")]
        client_secret: String,

        /// OAuth server with OIDC located at /.well-known/openid-configuration
        /// Do not include a trailing slash
        /// Defaults to https://pfapi.snowflake.blue
        #[arg(long, env = "ICEBLINK_OAUTH_SERVER")]
        oauth_server: Option<String>,
    },
}

pub fn get_settings() -> Cli {
    Cli::parse()
}
