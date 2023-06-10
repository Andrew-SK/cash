use std::{convert::Infallible, fmt::Display, ops::Deref, path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};

mod up_client;

#[derive(Clone, Debug)]
struct ConfigPath(PathBuf);

impl ConfigPath {
    fn from(s: &str) -> Self {
        ConfigPath(PathBuf::from(s))
    }
}

impl Display for ConfigPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl Deref for ConfigPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for ConfigPath {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ConfigPath(PathBuf::from(s)))
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// The target file in which to store transactions.
    ///
    /// If the file exists, it will be appended to. If it does not, it will be created
    #[arg(short, long, default_value_t = ConfigPath::from("~/.cash/transactions.yaml"))]
    store: ConfigPath,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetch new transactions from Up
    Fetch {
        /// Up Bank auth token
        #[arg(short, long)]
        up_token: String,
    },
    /// List transactions
    List,
    /// Show summary spending/saving over monthly periods
    Report,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch { up_token } => {
            handle_fetch(up_token, cli.store).await;
        }
        Commands::List => {
            todo!();
        }
        Commands::Report => {
            todo!();
        }
    };

    return;
}

/// Fetches transactions from the Up API and stores them in a file
async fn handle_fetch(up_token: String, _store_file: ConfigPath) {
    let client = up_client::UpClient::new(up_token);

    client.list_transactions(None, None).await;
}
