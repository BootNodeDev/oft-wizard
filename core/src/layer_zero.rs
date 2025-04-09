use std::sync::Arc;

use ethers::prelude::*;
use ethers::types::H256;

use crate::deployer::deploy_oapp_contract;
use crate::lz_options::build_options_with_lz_receive;
use crate::provider::get_providers_with_endpoints;

// TODO Make strings an enum of possible chains
pub async fn deploy_on_chains(
    chains: &[String],
    deployer: &LocalWallet,
) -> Result<(), anyhow::Error> {
    let mut addresses = Vec::new();

    println!("Start");
    let providers = get_providers_with_endpoints()?;

    // Deploy the oapp on each chain
    for chain in chains {
        println!("Deploying MyOApp on chain {}", chain);

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
        addresses.push((chain.clone(), oapp_addr, wallet));
    }

    abigen!(MyOApp, "core/src/solidity/artifacts/MyOApp.sol/MyOApp.json");

    // Set up peer connections (each with n-1 others)
    for (i, (chain_i, addr_i, local_sender)) in addresses.iter().enumerate() {
        for (j, (chain_j, addr_j, _)) in addresses.iter().enumerate() {
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
                    local_sender.clone(),
                ));

                let oapp_contract = MyOApp::new(*addr_i, client.clone());

                // Convert the address to the 20-byte format required by LayerZero
                let peer_addr = H256::from(*addr_j).to_fixed_bytes();
                let eid = match chain_j.as_str() {
                    "base_sepolia" => 40245,
                    "optimism_sepolia" => 40232,
                    _ => 1,
                };
                // oapp_contract
                println!("Set peer for {} with value {}", chain_i, eid);
                oapp_contract.set_peer(eid, peer_addr).send().await?;
                println!(
                    "Peers {:?} for {} in {}",
                    oapp_contract.peers(eid).call().await?,
                    addr_i,
                    chain_i
                );
                println!("{:?}", client.get_chainid().await?);
            }
        }
    }

    for (i, (chain_i, addr_i, local_sender)) in addresses.iter().enumerate() {
        for (j, (chain_j, _addr_j, _)) in addresses.iter().enumerate() {
            if i != j {
                println!("Sending message from {} to {} started", chain_i, chain_j);
                // Instantiate OApp contract using its address

                let rpc_info_for_current_chain = if let Some(rpc_info) = providers.get(chain_i) {
                    rpc_info.clone()
                } else {
                    println!("Chain {} not found in providers, skipping", chain_i);
                    continue;
                };

                let client = Arc::new(SignerMiddleware::new(
                    rpc_info_for_current_chain.clone().provider,
                    local_sender.clone(),
                ));
                let oapp_contract = MyOApp::new(*addr_i, client);

                // FIXME Add all this to object RpcInfo.
                let eid = match chain_j.as_str() {
                    "base_sepolia" => 40245,
                    "optimism_sepolia" => 40232,
                    _ => 1,
                };
                let message = format!("Hi from {}", chain_i);
                let adapter_params = Bytes::from(build_options_with_lz_receive(200_000, 0));

                // quote fee
                let quote_result = oapp_contract
                    .quote(eid, message.clone(), adapter_params.clone(), false)
                    .call()
                    .await?;

                let native_fee: U256 = quote_result.native_fee;

                // oapp_contract
                let tx_request = oapp_contract
                    .send(eid, message, adapter_params)
                    .value(native_fee);

                // FIXME await borrows?
                let tx = tx_request.send().await?;

                println!("Sending message from {} to {} finished", chain_i, chain_j);
                println!(
                    "Check message status on LayerZero Scan: https://testnet.layerzeroscan.com/tx/{:?}",
                    tx
                );
            }
        }
    }
    println!("Layer Zero network successfully deployed and connected!");
    Ok(())
}
