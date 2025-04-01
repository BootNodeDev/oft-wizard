use anyhow::Result;
use ethers::signers::{LocalWallet, Wallet};
use std::path::Path;

/// Load a wallet from a keystore file.
/// `cast wallet new .`
// comment example
pub async fn wallet_from_keystore(path: &str, password: &str) -> Result<LocalWallet> {
    let path = Path::new(path);
    let wallet = Wallet::decrypt_keystore(&path, password)?;
    println!("Wallet loaded successfully.");
    Ok(wallet)
}
