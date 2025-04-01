use crate::artifacts;
use anyhow::Result;
use ethers::{abi::Token, prelude::*};
use std::sync::Arc;

pub async fn deploy_contract(
    contract_name: &str,
    provider: Provider<Http>,
    wallet: LocalWallet,
) -> Result<Address> {
    let endpoint_address = "0x0000000000000000000000000000000000000001".parse::<Address>()?;
    let constructor_args = vec![
        abi::Token::Address(endpoint_address),
        abi::Token::Address(endpoint_address),
    ];

    let abi = artifacts::load_abi(contract_name)?;
    let bytecode = artifacts::load_bytecode(contract_name)?;
    println!("ABI {:?}\nBytecode {:?}", abi, bytecode);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let factory = ContractFactory::new(abi, bytecode, Arc::new(client.clone()));

    println!("Factory {:?}", factory);

    let deployed = factory.deploy(constructor_args)?.send().await?;

    Ok(deployed.address())
}
