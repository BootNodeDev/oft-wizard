use ethers::providers::{Http, Provider};
use rpassword::prompt_password;

/// Retrieves a wallet from a keystore file.
///
/// This function provides user prompting for password input when needed,
/// but delegates the actual keystore loading and wallet creation to the
/// `client::wallet_from_keystore` function which contains the core logic.
///
/// # Arguments
/// * `path` - Path to the keystore file
/// * `provider` - Optional Ethereum provider
/// * `password` - Optional password (will prompt user if None)
///
/// # Returns
/// A Result containing the loaded LocalWallet or an error
pub async fn get_wallet_from_keystore(
    path: &str,
    provider: Option<Provider<Http>>,
    password: Option<String>,
) -> anyhow::Result<ethers::signers::LocalWallet> {
    let password = match password {
        Some(pass) => pass,
        None => prompt_password("Enter password: ")?,
    };
    core::client::wallet_from_keystore(path, provider, &password).await
}
