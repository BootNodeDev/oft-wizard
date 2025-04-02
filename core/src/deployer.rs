use crate::artifacts;
use anyhow::Result;
use ethers::prelude::*;
use std::sync::Arc;

pub async fn deploy_contract(
    contract_name: &str,
    provider: Provider<Http>,
    wallet: LocalWallet,
) -> Result<Address> {
    // TODO Extract
    let endpoint_address = "0x6EDCE65403992e310A62460808c4b910D972f10f".parse::<Address>()?;
    let delegator_address = "0x6EDCE65403992e310A62460808c4b910D972f10f".parse::<Address>()?;

    // FIXME abi is not used, is replaced by abigen! macro. Remove from artifacts module
    // let abi = artifacts::load_abi(contract_name)?;
    // TODO Move closer to artifact reading
    abigen!(MyOApp, "core/src/solidity/artifacts/MyOApp.sol/MyOApp.json");

    // FIXME bytecode is not used, is replaced by abigen! macro. Remove from artifacts module
    // let bytecode = artifacts::load_bytecode(contract_name)?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Tuples are used.
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
