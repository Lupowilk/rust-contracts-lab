## Progress
Track learning via [GitHub Projects](https://github.com/Lupowilk/rust-contracts-lab/projects)

---

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

---

## Arbitrum Stylus
### Counter Contract (`arbitrum/hello-world`)
A basic counter contract written using `cargo-stylus`. Written and tested locally. Deployment skipped due to broken Sepolia faucets.

**Key Rust concepts covered:**
- `sol_storage!` macro for on-chain storage
- `#[public]` entrypoint
- `no_std` / `no_main` WASM environment

---

## Solana
### Hello World (`solana/hello-world`)
Minimal Anchor program that logs a message on-chain.

Program ID: `6wmDpCS3xyi6cxP22W4jjcSaNHBBJ5NAtECRYy8yaQGU`
Deployed on devnet.

### Token Contract (`solana/token-contract`)
Anchor program implementing SPL token minting.
- `create_mint` — creates a new SPL Mint account
- `mint_tokens` — mints tokens into a Token account

Program ID: `7XF9upHxdV5iVWehmVaAsPzzDwyrozdtgpEFFHbBtDTj`
Deployed on devnet. Tokens minted and verified on-chain.

**Key concepts covered:**
- Solana accounts model vs Ethereum and Cardano
- Programs vs contracts
- CPI (Cross Program Invocation)
- Anchor macros — `#[program]`, `#[derive(Accounts)]`, `#[account]`
- SPL Token Program
- Rust lifetimes and generics in practice

**Deploy:**
```bash
anchor build && anchor deploy
```

**Run client:**
```bash
cd app && node client.js
```

---

## Environment
- Rust: stable
- IDE: Zed
- Chains: NEAR, Arbitrum Stylus, Solana
