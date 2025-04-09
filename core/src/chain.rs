use ethers::types::Address;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedChain {
    BaseSepolia,
    OptimismSepolia,
    ArbitrumSepolia,
    GnosisChiado,
    LineaSepolia,
}

impl std::fmt::Display for SupportedChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedChain::BaseSepolia => write!(f, "base_sepolia"),
            SupportedChain::OptimismSepolia => write!(f, "optimism_sepolia"),
            SupportedChain::ArbitrumSepolia => write!(f, "arbitrum_sepolia"),
            SupportedChain::GnosisChiado => write!(f, "gnosis_chiado"),
            SupportedChain::LineaSepolia => write!(f, "linea_sepolia"),
        }
    }
}

impl FromStr for SupportedChain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "base_sepolia" => Ok(SupportedChain::BaseSepolia),
            "optimism_sepolia" => Ok(SupportedChain::OptimismSepolia),
            "arbitrum_sepolia" => Ok(SupportedChain::ArbitrumSepolia),
            "gnosis_chiado" => Ok(SupportedChain::GnosisChiado),
            "linea_sepolia" => Ok(SupportedChain::LineaSepolia),
            _ => Err(format!("Unsupported chain alias: {}", s)),
        }
    }
}

impl SupportedChain {
    pub fn lz_endpoint_id(&self) -> u32 {
        match self {
            SupportedChain::BaseSepolia => 40245,
            SupportedChain::OptimismSepolia => 40232,
            SupportedChain::ArbitrumSepolia => 40231,
            SupportedChain::GnosisChiado => 40145,
            SupportedChain::LineaSepolia => 40287,
        }
    }
    pub fn endpoint_address(&self) -> Address {
        match self {
            SupportedChain::BaseSepolia
            | SupportedChain::OptimismSepolia
            | SupportedChain::ArbitrumSepolia
            | SupportedChain::GnosisChiado
            | SupportedChain::LineaSepolia => "0x6EDCE65403992e310A62460808c4b910D972f10f"
                .parse::<Address>()
                .unwrap(),
        }
    }
}
