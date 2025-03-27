use crate::artifacts;
use anyhow::Result;
use ethers::prelude::*;
use std::sync::Arc;

pub async fn deploy_contract(
    contract_name: &str,
    provider: Provider<Http>,
    wallet: LocalWallet,
) -> Result<Address> {
    let abi = artifacts::load_abi(contract_name)?;
    let bytecode = artifacts::load_bytecode(contract_name)?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let factory = ContractFactory::new(abi, Bytes::from(bytecode.as_bytes().to_vec()), client);
    let deployer = factory.deploy(())?.send().await?;

    Ok(deployer.address())
}
