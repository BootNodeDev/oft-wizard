use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "deploy-cli")]
#[command(about = "Deploy contracts and manage deployer", long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub chain: String,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Wallet {
        #[command(subcommand)]
        action: WalletAction,
    },
    Deploy {
        #[arg(long)]
        path: String,
        #[arg(long, hide = true)]
        password: Option<String>,
        contract: String,
    },
    Compile,
}

#[derive(Subcommand)]
pub enum WalletAction {
    New,
    Balance {
        #[arg(long)]
        path: String,
        #[arg(long, hide = true)]
        password: Option<String>,
    },
}
