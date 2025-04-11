# OFT Wizard

## Overview

OFT Wizard is a project by [Bootnode](https://bootnode.dev) designed to simplify the development, deployment, and management of Omnichain Apps (OApp) applications using the LayerZero protocol. The project aims to provide more user-friendly tooling without requiring configuration files and plaintext private keys.

## Key Features

- **Core Library**: Shared logic for deploying, configuring, and interacting with OApps
- **CLI Tool**: Command-line interface for basic deployment and wallet management
- **Wallet Management**: Secure wallet handling without plaintext private keys
- **Cross-Chain Messaging**: Built-in support for sending arbitrary messages between deployed OApps

## Design Philosophy

The primary goal of OFT Wizard is to provide developer-friendly tools that abstract away the complexity of:

- RPC configuration and management
- Wallet instantiation and security
- Peer configuration
- Message fee calculation
- Bytes and options handling for LayerZero protocol

All of these complexities are hidden from the user, allowing for a streamlined development experience.

## Project Structure

- **core**: Core library containing shared logic and abstractions
- **cli**: Command-line interface for basic operations

## Prerequisites

### Solidity Compiler

This project uses the Solidity compiler via SVM (Solidity Version Manager). Please refer to the [core README](./core/README.md) for detailed installation instructions.

### RPC Endpoints

The project requires RPC endpoints for the supported chains. Currently, these are configured in the Foundry configuration (`foundry.toml`).

**Note**: This approach requires improvement as it currently requires API keys (like Alchemy tokens) to be stored in plaintext. Future versions will address this security concern.

## Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/bootnode/oft_wizard.git
cd oft_wizard

# Build the project
cargo build --release
```

### Using the CLI

```bash
# Compile contracts
./target/release/cli compile

# Create a new wallet
./target/release/cli --chain base_sepolia wallet new

# Check wallet balance
./target/release/cli --chain base_sepolia wallet balance --path /path/to/wallet

# Deploy to multiple chains
./target/release/cli --chain base_sepolia deploy --path /path/to/wallet --chainlist optimism_sepolia arbitrum_sepolia
```

## Supported Chains

- Base Sepolia
- Optimism Sepolia
- Arbitrum Sepolia
- Gnosis Chiado

## Advanced Features

### Message Types

The example OApp from the LayerZero quickstart demonstrates string message passing, but the framework supports more complex use cases:

- **Unicast**: Send messages to a single destination
- **Multicast**: Send messages to multiple specified destinations
- **Broadcast**: Send messages to all peer contracts

These patterns can be used to implement various cross-chain applications beyond simple mint/burn token operations.

## Future Roadmap

- REST API implementation using Axum
- Support for additional blockchains compatible with LayerZero (Solana, Aptos, etc.)
- Enhanced contract customization options
- Improved security for API key management
- Event listener agents for monitoring cross-chain messages

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This is an experimental project. Use at your own risk.
