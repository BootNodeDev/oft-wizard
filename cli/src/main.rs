use clap::{Parser, Subcommand};
use core::compiler::compile_contract;
use core::{client, deployer};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::Signer;
use ethers::types::Address;
use foundry_config::Config;
use rpassword::prompt_password;
use std::process::Command;

#[derive(Parser)]
#[command(name = "deploy-cli")]
#[command(about = "Deploy contracts and manage deployer", long_about = None)]
struct Cli {
    #[arg(long)]
    chain: String,

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
        #[arg(long)]
        path: String,
        #[arg(long, hide = true)]
        password: Option<String>,
        contract: String,
    },
    Compile,
}

#[derive(Subcommand)]
enum WalletAction {
    New,
    Balance {
        #[arg(long)]
        path: String,
        #[arg(long, hide = true)]
        password: Option<String>,
    },
}

async fn get_wallet_from_keystore(
    path: &str,
    password: Option<String>,
) -> anyhow::Result<ethers::signers::LocalWallet> {
    let password = match password {
        Some(pass) => pass,
        None => prompt_password("Enter password: ")?,
    };
    client::wallet_from_keystore(path, &password).await
}

fn get_provider(chain_alias: &str) -> anyhow::Result<Provider<Http>> {
    let config = Config::load();

    let rpc_url: String = config
        .get_rpc_url_with_alias(chain_alias)
        .ok_or_else(|| anyhow::anyhow!("Error trying to get RPC URL from config"))?
        .map_err(|e| anyhow::anyhow!("Error getting RPC URL: {}", e))?
        .into_owned();

    Provider::<Http>::try_from(rpc_url)
        .map_err(|e| anyhow::anyhow!("Error instantiating provider: {}", e))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let chain = cli.chain;
    let provider = get_provider(chain.as_str()).expect("Provider setup failed");

    match cli.command {
        Commands::Wallet { action } => match action {
            WalletAction::New => {
                let output = Command::new("cast")
                    .arg("wallet")
                    .arg("new")
                    .arg("wallets")
                    .output()?;

                println!(
                    "Error from cast wallet new {:?}",
                    String::from_utf8_lossy(&output.stderr)
                );
                println!(
                    "Output from cast wallet new {:?}",
                    String::from_utf8_lossy(&output.stdout)
                );
            }
            WalletAction::Balance { path, password } => {
                let wallet = get_wallet_from_keystore(path.as_str(), password).await?;
                let addr: Address = wallet.address();
                let balance = provider.get_balance(addr, None).await?;
                println!(
                    "Deployer: {:?}\nBalance: {} ETH",
                    addr,
                    ethers::utils::format_ether(balance)
                );
            }
        },
        Commands::Deploy {
            path,
            password,
            contract,
        } => {
            let wallet = get_wallet_from_keystore(path.as_str(), password).await?;
            let addr = deployer::deploy_contract(&contract, provider, wallet).await?;
            println!("Deployed `{}` at {:?}\n", contract, addr);
        }
        Commands::Compile => {
            println!("Compiling contract...");
            match compile_contract() {
                Ok(_) => println!("Compilation complete"),
                Err(e) => println!("Compilation failed: {}", e),
            };
        }
    }

    Ok(())
}
