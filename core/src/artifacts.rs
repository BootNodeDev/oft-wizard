use anyhow::{Context, Result};
use ethers::abi::Abi;
use std::fs;

fn contract_path(contract_name: &str) -> String {
    format!(
        "contracts/{}/out/{}.sol/{}.json",
        contract_name, contract_name, contract_name
    )
}

pub fn load_abi(contract_name: &str) -> Result<Abi> {
    let path = contract_path(contract_name);
    let json =
        fs::read_to_string(&path).with_context(|| format!("failed to read abi from {}", path))?;
    let v: serde_json::Value = serde_json::from_str(&json)?;
    let abi_str = serde_json::to_string(&v["abi"])?;
    let abi = serde_json::from_str(&abi_str)?;
    Ok(abi)
}

pub fn load_bytecode(contract_name: &str) -> Result<String> {
    let path = contract_path(contract_name);
    let json = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read bytecode from {}", path))?;
    let v: serde_json::Value = serde_json::from_str(&json)?;
    let bytecode = v["bytecode"]["object"]
        .as_str()
        .context("Missing bytecode object")?
        .to_string();
    Ok(bytecode)
}
