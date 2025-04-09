use std::collections::HashMap;

use ethers::prelude::*;
use ethers::types::H256;
use futures::future::join_all;

use crate::chain::SupportedChain;
use crate::deployer::deploy_oapp_contract;
use crate::lz_options::build_options_with_lz_receive;
use crate::provider::build_chain_clients;

abigen!(MyOApp, "core/src/solidity/artifacts/MyOApp.sol/MyOApp.json");
pub async fn deploy_on_chains(
    chains: &[SupportedChain],
    deployer: &LocalWallet,
) -> Result<HashMap<SupportedChain, H160>, anyhow::Error> {
    let chain_clients = build_chain_clients(deployer).await?;

    // Deploy the oapp on each chain using tokio
    let mut chain_addresses = HashMap::new();

    let futures = chains.iter().map(|chain| {
        let chain_client = chain_clients.get(chain).unwrap().clone();
        let chain_clone = chain.clone();
        async move {
            println!("Deploying MyOApp on chain {}", chain_clone);
            let oapp_addr = deploy_oapp_contract(&chain_clone, &chain_client).await?;
            Ok::<(SupportedChain, H160), anyhow::Error>((chain_clone, oapp_addr))
        }
    });

    let results = join_all(futures).await;

    for result in results {
        let (chain, addr) = result?;
        chain_addresses.insert(chain, addr);
    }

    Ok(chain_addresses)
}

pub async fn setup_peer_connections(
    deployer: &LocalWallet,
    addresses: &HashMap<SupportedChain, H160>,
) -> Result<(), anyhow::Error> {
    let chain_clients = build_chain_clients(deployer).await?;

    // Set up peer connections (each with n-1 others)
    for chain_i in addresses.keys() {
        for chain_j in addresses.keys() {
            if chain_i != chain_j {
                println!("Connecting {} to {}", chain_i, chain_j);

                let addr_i = addresses.get(chain_i).unwrap();
                let addr_j = addresses.get(chain_j).unwrap();
                let client = chain_clients.get(chain_i).unwrap().clone();

                // Instantiate OApp contract using its address
                let oapp_contract = MyOApp::new(*addr_i, client.clone());

                // Convert the address to the 20-byte format required by LayerZero
                let peer_addr = H256::from(*addr_j).to_fixed_bytes();
                let eid = chain_j.lz_endpoint_id();

                // oapp_contract
                println!("Set peer for {} with value {}", chain_i, eid);
                oapp_contract.set_peer(eid, peer_addr).send().await?;
            }
        }
    }
    Ok(())
}

pub async fn send_cross_chain_message(
    deployer: &LocalWallet,
    addresses: &HashMap<SupportedChain, H160>,
    source_chain: &SupportedChain,
    destination_chain: &SupportedChain,
    message: String,
) -> Result<H256, anyhow::Error> {
    if source_chain == destination_chain {
        return Err(anyhow::anyhow!(
            "Source and destination chains must be different"
        ));
    }

    if !addresses.contains_key(source_chain) || !addresses.contains_key(destination_chain) {
        return Err(anyhow::anyhow!(
            "Source or destination chain not found in addresses"
        ));
    }

    let chain_clients = build_chain_clients(deployer).await?;
    let source_addr = addresses.get(source_chain).unwrap();
    let client = chain_clients.get(source_chain).unwrap().clone();

    // Instantiate OApp contract using its address
    let oapp_contract = MyOApp::new(*source_addr, client);

    // Get the endpoint ID for the destination chain
    let eid = destination_chain.lz_endpoint_id();

    // Prepare adapter params
    let adapter_params = Bytes::from(build_options_with_lz_receive(200_000, 0));

    // Quote the fee
    let quote_result = oapp_contract
        .quote(eid, message.clone(), adapter_params.clone(), false)
        .call()
        .await?;

    let native_fee: U256 = quote_result.native_fee;

    // Send the message
    println!(
        "Sending message from {} to {}",
        source_chain, destination_chain
    );
    let tx = oapp_contract
        .send(eid, message, adapter_params)
        .value(native_fee);

    let tx_hash = tx.send().await?.tx_hash();

    println!(
        "Message sent from {} to {}. Transaction hash: {:?}",
        source_chain, destination_chain, tx_hash
    );
    println!(
        "Check message status on LayerZero Scan: https://testnet.layerzeroscan.com/tx/{:?}",
        tx_hash
    );

    Ok(tx_hash)
}

pub async fn send_messages_to_all_chains(
    deployer: &LocalWallet,
    addresses: &HashMap<SupportedChain, H160>,
    source_chain: &SupportedChain,
    message: String,
) -> Result<Vec<H256>, anyhow::Error> {
    if !addresses.contains_key(source_chain) {
        return Err(anyhow::anyhow!("Source chain not found in addresses"));
    }

    let futures = addresses
        .keys()
        .filter(|&chain| chain != source_chain)
        .map(|dest_chain| {
            let deployer_clone = deployer.clone();
            let addresses_clone = addresses.clone();
            let source_chain_clone = source_chain.clone();
            let dest_chain_clone = dest_chain.clone();
            let message_clone = message.clone();

            async move {
                send_cross_chain_message(
                    &deployer_clone,
                    &addresses_clone,
                    &source_chain_clone,
                    &dest_chain_clone,
                    message_clone,
                )
                .await
            }
        });

    let results = join_all(futures).await;

    // Collect successful transaction hashes
    let mut tx_hashes = Vec::new();
    for result in results {
        match result {
            Ok(hash) => tx_hashes.push(hash),
            Err(e) => println!("Error sending message: {}", e),
        }
    }

    Ok(tx_hashes)
}
