# NEAR Development Setup

## Prerequisites
- Rust toolchain
- Cargo

## Installation Steps

1. Install NEAR CLI via cargo:
```bash
cargo install near-cli-rs
```

2. Add WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Create testnet account:
```bash
near account create-account sponsor-by-faucet-service <your-name>.testnet autogenerate-new-keypair save-to-keychain network-config testnet create
```

## Verification

- NEAR CLI: `near --version`
- Testnet account: `lupo-dev.testnet`
- WASM target: `rustup target list --installed | grep wasm32`

## Resources
- [NEAR Docs](https://docs.near.org)
- [NEAR CLI Docs](https://docs.near.org/tools/near-cli)
