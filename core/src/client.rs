use anyhow::{Context, Result};
use ethers::signers::{LocalWallet, Wallet};
use std::env;
use std::fs;

pub async fn wallet_from_env() -> Result<LocalWallet> {
    let key = env::var("PRIVATE_KEY").context("PRIVATE_KEY not set")?;
    Ok(key.parse::<LocalWallet>()?)
}

/// Load a wallet from a keystore file.
/// `cast wallet new .`
pub async fn wallet_from_keystore(path: &str, password: &str) -> Result<LocalWallet> {
    let json = fs::read_to_string(path)?;
    let wallet = Wallet::decrypt_keystore(&json, password)?;
    Ok(wallet)
}
