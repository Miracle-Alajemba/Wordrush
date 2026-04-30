# WordPot Ink! Smart Contract

This directory contains the Polkadot-based WordPot smart contract written in Ink! (Rust).

## Setup Instructions

### Prerequisites
- Rust toolchain (install from https://rustup.rs/)
- Cargo contract tool: `cargo install cargo-contract`
- ink! library support

### Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-contract
cargo install cargo-contract

# Build the contract
cargo contract build

# Run tests
cargo contract test
```

## Contract Functions

### GameRoom Management
- `create_room(entry_fee: u128, round_duration: u64) -> RoomId`
- `join_room(room_id: RoomId, player_address: AccountId) -> Result`
- `submit_word(room_id: RoomId, word: String) -> Result`
- `claim_reward(room_id: RoomId, player_address: AccountId) -> Result`
- `cancel_room(room_id: RoomId, player_addresses: Vec<AccountId>) -> Result`

## Deployment

```bash
# Deploy to Portaldot testnet
cargo contract upload --manifest-path contracts/ink/Cargo.toml
```

## TODO
- [ ] Define Ink! contract structure
- [ ] Implement game room logic
- [ ] Implement reward distribution
- [ ] Add access control
- [ ] Write tests
- [ ] Deploy to testnet
- [ ] Verify contract on explorer
