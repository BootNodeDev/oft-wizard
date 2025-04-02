use anyhow::Result;
use ethers::signers::{LocalWallet, Signer, Wallet};
use std::path::Path;

/// Load a wallet from a keystore file.
/// `cast wallet new .`
// comment example
pub async fn wallet_from_keystore(path: &str, password: &str) -> Result<LocalWallet> {
    let path = Path::new(path);
    let mut wallet = Wallet::decrypt_keystore(&path, password)?;

    // TODO Replace with chain id from provider
    let chain_id: u64 = 84532;
    wallet = wallet.with_chain_id(chain_id);

    println!("Wallet loaded successfully.");
    Ok(wallet)
}
