# Rust Contracts Lab

Learning Rust smart contracts across Arbitrum, Solana, and NEAR.

## Structure

```
arbitrum/    # EVM-compatible contracts (Stylus)
solana/      # Solana programs
near/        # NEAR Protocol contracts
docs/        # Learning notes and references
```

## Progress

Track learning via [GitHub Projects](https://github.com/Lupowilk/rust-contracts-lab/projects)

## NEAR

### Token Contract (`near/token`)

A basic fungible token contract implementing:

- `init_token_supply` — initializes the contract and assigns total supply to the deployer
- `balance_of` — returns the balance of a given account
- `total_supply` — returns the total token supply
- `transfer` — transfers tokens between accounts with balance validation

Deployed on testnet at `chrupciatoken.testnet`

**Key Rust concepts covered:**
- `HashMap` for on-chain key-value storage
- `Option` types and pattern matching
- Ownership, borrowing, and dereferencing
- Unit testing with `VMContextBuilder` and mock blockchain context

**Run tests:**
```bash
cd near/token
cargo test
```

**Build:**
```bash
cargo near build non-reproducible-wasm
```

**Deploy:**
```bash
cargo near deploy build-non-reproducible-wasm
```

## Environment

- Rust: 1.86.0 (required for NEAR contract compilation)
- IDE: Zed
- Chains: Arbitrum Stylus, Solana, NEAR

## Getting Started

_Will be populated as we progress._
