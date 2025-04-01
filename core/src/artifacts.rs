use anyhow::{Context, Result};
use ethers::{abi::Abi, types::Bytes, utils::hex::decode};
use std::fs;

fn contract_path(contract_name: &str) -> String {
    let artifacts_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/solidity/artifacts");

    format!(
        "{}/{}.sol/{}.json",
        artifacts_dir.display(),
        contract_name,
        contract_name
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

pub fn load_bytecode(contract_name: &str) -> Result<Bytes> {
    let path = contract_path(contract_name);
    let json = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read bytecode from {}", path))?;
    let v: serde_json::Value = serde_json::from_str(&json)?;
    let bytecode_str = v["bytecode"]["object"]
        .as_str()
        .context("Missing bytecode object")?;
    let bytecode = decode(bytecode_str.strip_prefix("0x").unwrap_or(bytecode_str))?;
    Ok(Bytes::from(bytecode))
}
