use anyhow::Result;
use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
};
use std::path::Path;

/// Load a wallet from a keystore file.
/// `cast wallet new .` is needed to call from CLI first.
pub async fn wallet_from_keystore(
    path: &str,
    provider: Option<Provider<Http>>,
    password: &str,
) -> Result<LocalWallet> {
    let path = Path::new(path);
    let mut wallet = Wallet::decrypt_keystore(&path, password)?;

    match provider {
        Some(provider) => {
            let chain_id = provider.get_chainid().await?.as_u64();
            wallet = wallet.with_chain_id(chain_id);
        }
        None => (),
    }

    println!("Wallet loaded successfully.");
    Ok(wallet)
}
