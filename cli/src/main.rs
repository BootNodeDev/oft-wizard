pub mod cli;
pub mod helpers;

use crate::cli::{Cli, Commands, WalletAction};
use clap::Parser as _;
use core::chain::SupportedChain;
use core::compiler::compile_contract;
use core::layer_zero::{deploy_on_chains, setup_peer_connections};
use core::provider::build_chain_clients;
use ethers::providers::Middleware;
use ethers::signers::Signer;
use ethers::types::Address;
use std::process::Command;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();
    let chain = cli.chain;

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
                let wallet =
                    helpers::get_wallet_from_keystore(path.as_str(), None, password).await?;

                let addr: Address = wallet.address();
                let chain_object = SupportedChain::from_str(&chain).unwrap();
                let clients = build_chain_clients(&wallet).await?;
                let provider = clients.get(&chain_object).unwrap().provider();
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
            chainlist,
        } => {
            let wallet = helpers::get_wallet_from_keystore(path.as_str(), None, password).await?;

            let supported_chains = chainlist
                .iter()
                .map(|chain_str| {
                    chain_str
                        .parse::<core::chain::SupportedChain>()
                        .expect("Invalid chain")
                })
                .collect::<Vec<core::chain::SupportedChain>>();

            let deployed = deploy_on_chains(&supported_chains, &wallet).await?;
            for (chain, addr) in supported_chains.iter().zip(deployed.iter()) {
                println!("Chain {}: Deployed contract at address: {:?}", chain, addr);
            }

            // TODO make this optional
            setup_peer_connections(&wallet, &deployed).await?;
            // Send cross-chain message from each chain to every other chain
            println!("Sending cross-chain messages...");
            for (src_idx, src_chain) in supported_chains.iter().enumerate() {
                for (dst_idx, dst_chain) in supported_chains.iter().enumerate() {
                    if src_idx != dst_idx {
                        let message = format!("Hi {} from {}", dst_chain, src_chain);
                        match core::layer_zero::send_cross_chain_message(
                            &wallet, &deployed, src_chain, dst_chain, message,
                        )
                        .await
                        {
                            Ok(_) => println!(
                                "Successfully sent message from {} to {}",
                                src_chain, dst_chain
                            ),
                            Err(e) => println!(
                                "Failed to send message from {} to {}: {}",
                                src_chain, dst_chain, e
                            ),
                        }
                    }
                }
            }
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
