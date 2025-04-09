use anyhow::Result;
use ethers::prelude::*;

use crate::{chain::SupportedChain, provider::ChainClient};

// FIXME Hardcoded string
abigen!(MyOApp, "core/src/solidity/artifacts/MyOApp.sol/MyOApp.json");

pub async fn deploy_oapp_contract(
    chain: &SupportedChain,
    chain_client: &ChainClient,
) -> Result<Address> {
    let endpoint_address = chain.endpoint_address();
    let delegator_address = chain_client.address();

    let deployed = MyOApp::deploy(chain_client.clone(), (endpoint_address, delegator_address))
        .map_err(|e| {
            println!("Deployment preparation error: {:?}", e);
            anyhow::Error::from(e)
        })?
        .send()
        .await
        .map_err(|e| {
            println!("Deployment execution error: {:?}", e);
            anyhow::Error::from(e)
        })?;

    Ok(deployed.address())
}
