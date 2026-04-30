# WordPot - Portaldot Edition

WordPot is a multiplayer word game built on Polkadot (via Portaldot). Players join live rooms,
make words from a shared source word, and compete for a score-based share of the
reward pool. This version brings WordPot to the Polkadot ecosystem.

## Project Status

🚀 **Building for Portaldot Mini Hackathon (Deadline: May 31, 2026)**

Original Celo version: https://wordpot.vercel.app/

## Stack

- `client/`: React 19 + Vite + @polkadot/api
- `server/`: Node.js + Express + @polkadot/api
- `contracts/`: Ink! (Rust) + cargo-contract
- `network`: Portaldot (Polkadot ecosystem)

## Game Features

- Multiplayer real-time word game
- Polkadot wallet connection (Polkadot.js browser extension)
- Onchain room entry with POT (Portaldot token)
- Score-based reward distribution
- Treasury fee system (10% treasury, 90% reward pool)
- 60-second game rounds

## Development Roadmap

**Phase 1** (Day 1): Project Setup ✅
- Updated dependencies (wagmi → @polkadot/api)
- Created Ink! contract directory structure
- Created Portaldot configuration files

**Phase 2** (Days 2-4): Ink! Smart Contract
- Define contract structure
- Implement game room logic
- Deploy to testnet

**Phase 3** (Days 5-6): Polkadot Wallet Integration
- Replace Reown AppKit with Polkadot.js
- Implement wallet connection flow

**Phase 4** (Days 7-8): Backend Adaptation
- Update contract interaction layer
- Refactor for Polkadot API

**Phase 5** (Days 9-10): Frontend Adaptation
- Update UI for Portaldot
- Integrate Polkadot wallet

**Phase 6** (Days 11-12): Testing & QA

**Phase 7** (Days 13-14): Hackathon Submission

## Getting Started

```bash
# Install dependencies
cd client && npm install
cd ../server && npm install

# Setup environment
cp .env.portaldot .env.local

# Development
cd client && npm run dev
cd ../server && npm run dev
```

## Original Celo Version

- App: https://wordpot.vercel.app/
- Contract: `0x764b3f8761CEB44e6FFA6480484b706C3c3A8284`
- Mainnet: Celo
   - supports score-based reward claims after settlement

## Project Structure

- `client/` React frontend
- `server/` Express backend and room logic
- `contracts/` Solidity scaffold for escrow and claims
- `docs/` product and rules notes

## Prize Model

- Offchain game entry display: `0.1 cUSD`
- Treasury cut: `10%`
- Reward pool: `90%`
- Payout formula:

`(player score / total room score) × reward pool`

## Environment

Create `server/.env` from `server/.env.example` and set:

- `TREASURY_WALLET`
- `WORDPOT_CONTRACT_ADDRESS`
- `CELO_CHAIN_ID`
- `JOIN_PAYMENT_WEI`
- `JOIN_PAYMENT_DISPLAY`

If `WORDPOT_CONTRACT_ADDRESS` is not set yet, the lobby uses the treasury beta
join-payment flow while payout remains preview-only in the UI.

## Run Locally

### Client

```bash
cd client
npm install
npm run dev
```

### Server

```bash
cd server
npm install
cp .env.example .env
npm run dev
```

## Next Upgrade

- deploy `WordPotArena.sol`
- switch join flow from treasury beta to contract entry
- add claim reward transaction flow in the results screen
- move room sync from polling to sockets
