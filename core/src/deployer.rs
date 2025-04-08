use crate::provider::RpcInfo;
use anyhow::Result;
use ethers::prelude::*;
use std::sync::Arc;

pub async fn deploy_oapp_contract(rpc_info: RpcInfo, wallet: LocalWallet) -> Result<Address> {
    let endpoint_address = rpc_info.endpoint;
    let delegator_address = wallet.address();

    // FIXME Hardcoded string
    abigen!(MyOApp, "core/src/solidity/artifacts/MyOApp.sol/MyOApp.json");

    let client = Arc::new(SignerMiddleware::new(rpc_info.clone().provider, wallet));

    let deployed = MyOApp::deploy(client, (endpoint_address, delegator_address))
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
