use clap::{Parser, Subcommand};
use core::{client, deployer};
use ethers::providers::{Http, Provider};
// use ethers::types::Address;
use foundry_config::Config;
use std::process::Command;

#[derive(Parser)]
#[command(name = "deploy-cli")]
#[command(about = "Deploy contracts and manage deployer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Wallet {
        #[command(subcommand)]
        action: WalletAction,
    },
    Deploy {
        contract: String,
    },
}

#[derive(Subcommand)]
enum WalletAction {
    New,
    Balance,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::load_with_root("./");
    println!("{:?}", config);
    let rpc_url: String = match config.get_rpc_url_with_alias("optimism_sepolia") {
        Some(Ok(rpc)) => rpc.into_owned(),
        _ => panic!("rpc"),
    };

    let provider = match Provider::<Http>::try_from(rpc_url) {
        Ok(provider) => provider,
        _ => panic!("provider"),
    };

    match cli.command {
        Commands::Wallet { action } => match action {
            WalletAction::New => {
                let output = Command::new("cast")
                    .arg("wallet")
                    .arg("new")
                    .arg("../wallets")
                    .output()?;
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            WalletAction::Balance => {
                let wallet = client::wallet_from_env().await?;
                // let addr: Address = wallet.address();
                // let balance = provider.get_balance(addr, None).await?;
                // println!(
                //     "Deployer: {}\nBalance: {} ETH",
                //     addr,
                //     ethers::utils::format_ether(balance)
                // );
            }
        },
        Commands::Deploy { contract } => {
            let wallet = client::wallet_from_env().await?;
            let addr = deployer::deploy_contract(&contract, provider, wallet).await?;
            println!("Deployed `{}` at {}\n", contract, addr);
        }
    }

    Ok(())
}
