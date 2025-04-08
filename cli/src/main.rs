pub mod cli;
pub mod helpers;

use crate::cli::{Cli, Commands, WalletAction};
use clap::Parser as _;
use core::compiler::compile_contract;
use core::deployer;
use core::provider::get_providers_with_endpoints;
use ethers::providers::Middleware;
use ethers::signers::Signer;
use ethers::types::Address;
use std::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let chain = cli.chain;

    let providers = match get_providers_with_endpoints() {
        Ok(providers) => providers,
        Err(e) => return Err(e),
    };

    let rpc_info = match providers.get(chain.as_str()) {
        Some(rpc_info) => rpc_info.clone(),
        None => return Err(anyhow::anyhow!("Chain '{}' not found", chain)),
    };

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
                let wallet = helpers::get_wallet_from_keystore(
                    path.as_str(),
                    Some(rpc_info.clone().provider),
                    password,
                )
                .await?;

                let addr: Address = wallet.address();
                let balance = rpc_info.provider.get_balance(addr, None).await?;

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
            let wallet = helpers::get_wallet_from_keystore(
                path.as_str(),
                Some(rpc_info.provider.clone()),
                password,
            )
            .await?;
            let addr = deployer::deploy_oapp_contract(rpc_info, wallet).await?;
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
