use ethers::providers::{Http, Provider};
use ethers::types::Address;
use foundry_config::Config;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RpcInfo {
    pub provider: Provider<Http>,
    pub endpoint: Address,
}

pub fn get_providers_with_endpoints() -> anyhow::Result<HashMap<String, RpcInfo>> {
    let config = Config::load();

    let rpc_endpoints = config.rpc_endpoints.clone();
    let mut rpc_info: HashMap<String, RpcInfo> = HashMap::new();

    for (alias, raw_url) in rpc_endpoints.iter() {
        let provider = Provider::<Http>::try_from(raw_url.to_string().clone())
            .map_err(|e| anyhow::anyhow!("Error instantiating provider: {}", e))?;

        let endpoint_default_address =
            "0x6EDCE65403992e310A62460808c4b910D972f10f".parse::<Address>()?;

        let endpoint = if let Some(addr_str) =
            std::env::var(format!("ENDPOINT_{}", alias.to_uppercase())).ok()
        {
            addr_str
                .parse::<Address>()
                .map_err(|_| anyhow::anyhow!("Invalid endpoint address format for {}", alias))?
        } else {
            endpoint_default_address
        };

        rpc_info.insert(alias.clone(), (RpcInfo { provider, endpoint }).clone());
    }

    // let result = match rpc_info.get(chain_alias) {
    //     Some(rpc_info) => Ok(rpc_info.clone()),
    //     None => Err(anyhow::anyhow!("Provider not found for {}", chain_alias)),
    // };

    Ok(rpc_info)
}
