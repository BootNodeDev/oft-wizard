use std::sync::Arc;

use ethers::prelude::*;
use ethers::types::H256;

use crate::deployer::deploy_oapp_contract;
use crate::provider::get_providers_with_endpoints;

// TODO Make strings an enum of possible chains
pub async fn deploy_on_chains(
    chains: &[String],
    deployer: &LocalWallet,
) -> Result<(), anyhow::Error> {
    let mut addresses = Vec::new();

    let providers = get_providers_with_endpoints()?;

    // Deploy the oapp on each chain
    for chain in chains {
        println!("Deploying provider on chain {}", chain);

        let rpc_info_for_current_chain = if let Some(rpc_info) = providers.get(chain) {
            rpc_info.clone()
        } else {
            println!("Chain {} not found in providers, skipping", chain);
            continue;
        };

        let chain_id = rpc_info_for_current_chain
            .provider
            .get_chainid()
            .await?
            .as_u64();
        let wallet = deployer.clone().with_chain_id(chain_id);

        let oapp_addr =
            deploy_oapp_contract(rpc_info_for_current_chain.clone(), wallet.clone()).await?;
        addresses.push((chain.clone(), oapp_addr));
    }

    abigen!(MyOApp, "core/src/solidity/artifacts/MyOApp.sol/MyOApp.json");

    // Set up peer connections (each with n-1 others)
    for (i, (chain_i, addr_i)) in addresses.iter().enumerate() {
        for (j, (chain_j, addr_j)) in addresses.iter().enumerate() {
            if i != j {
                println!("Connecting {} to {}", chain_i, chain_j);
                // Instantiate OApp contract using its address

                let rpc_info_for_current_chain = if let Some(rpc_info) = providers.get(chain_i) {
                    rpc_info.clone()
                } else {
                    println!("Chain {} not found in providers, skipping", chain_i);
                    continue;
                };

                let client = Arc::new(SignerMiddleware::new(
                    rpc_info_for_current_chain.clone().provider,
                    deployer.clone(),
                ));
                let oapp_contract = MyOApp::new(*addr_i, client);

                // Convert the address to the 20-byte format required by LayerZero
                let peer_addr = H256::from(*addr_j).to_fixed_bytes();
                let eid = match chain_i.as_str() {
                    "base_sepolia" => 40245,
                    "optimism_sepolia" => 40232,
                    _ => 1,
                };
                // oapp_contract
                oapp_contract.set_peer(eid, peer_addr).await?;
            }
        }
    }

    println!("Layer Zero network successfully deployed and connected!");
    Ok(())
}
