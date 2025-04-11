use dotenvy;
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use foundry_config::{Config, RpcEndpoint};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use crate::chain::SupportedChain;

pub type ProviderHttp = Provider<Http>;
pub type WalletLocal = LocalWallet;
pub type ChainMiddleware = NonceManagerMiddleware<SignerMiddleware<ProviderHttp, WalletLocal>>;
pub type ChainClient = Arc<ChainMiddleware>;

pub fn get_rpc_endpoints_from_foundry_config()
-> anyhow::Result<HashMap<SupportedChain, RpcEndpoint>> {
    dotenvy::dotenv().ok(); // load .env

    let config = Config::load();

    let mut rpc_endpoints = HashMap::new();
    for (alias, endpoint) in config.rpc_endpoints.iter() {
        match SupportedChain::from_str(alias) {
            Ok(supported_chain) => {
                rpc_endpoints.insert(supported_chain, endpoint.clone());
            }
            Err(e) => {
                eprintln!("Chain alias is not supported: {}", e);
                continue;
            }
        }
    }
    Ok(rpc_endpoints)
}

pub async fn build_chain_clients(
    wallet: &LocalWallet,
) -> anyhow::Result<HashMap<SupportedChain, ChainClient>> {
    let rpc_endpoints = get_rpc_endpoints_from_foundry_config().unwrap();

    let mut clients: HashMap<SupportedChain, ChainClient> = HashMap::new();
    for (alias, raw_url) in rpc_endpoints.iter() {
        let provider = Provider::<Http>::try_from(raw_url.to_string().clone())
            .map_err(|e| anyhow::anyhow!("Error instantiating provider: {}", e))?;

        let chain_id = provider.get_chainid().await?.as_u64();

        let signer = SignerMiddleware::new(provider, wallet.clone().with_chain_id(chain_id));
        let address = signer.address();
        let nonce_manager = NonceManagerMiddleware::new(signer, address);

        let arc_client = Arc::new(nonce_manager);

        clients.insert(alias.clone(), arc_client);
    }

    Ok(clients)
}
