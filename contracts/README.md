# Portaldot WordPot Smart Contracts

This directory contains the Ink! smart contract for WordPot on Portaldot (Polkadot ecosystem).

## What The Contract Does

1. Manages game rooms and player entries
2. Holds room entry fees in POT (Portaldot native token)
3. Keeps treasury cut (10% of fees)
4. Stores player scores after settlement
5. Distributes rewards proportionally to players

## Technology Stack

- **Language**: Ink! (Rust-based DSL for Polkadot smart contracts)
- **Build Tool**: cargo-contract
- **Network**: Portaldot (Polkadot ecosystem)
- **Token**: POT (Portaldot native token)

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-contract
cargo install cargo-contract

# Verify installation
cargo contract --version
```

## Setup

Copy `.env.example` to `.env` and set:

- `PORTALDOT_RPC`: Portaldot RPC endpoint
- `CONTRACT_DEPLOYER_PRIVATE_KEY`: Deployer account key
- `TREASURY_WALLET`: Address to receive treasury fees

## Commands

```bash
# Build the contract
npm run build

# Run tests
npm run test

# Upload to Portaldot testnet
npm run upload

# Instantiate contract
npm run instantiate
```

## Deployment Flow

1. Run `npm run build` to compile Ink! contract
2. Run `npm run upload` to deploy to Portaldot testnet
3. Copy the deployed contract address
4. Set `VITE_CONTRACT_ADDRESS` in `client/.env.local`
5. Set `SERVER_CONTRACT_ADDRESS` in `server/.env`
6. Restart backend server
7. Test contract interaction from frontend

## Contract Interface

See [ink/README.md](./ink/README.md) for detailed contract implementation and function documentation.

