# ness-bc Documentation

Rust blockchain integration — Solana, Privy, x402.

## Tech Stack

- Rust 2024 edition
- solana-sdk / solana-client
- Privy Rust SDK (server-side wallet operations)
- x402 protocol implementation

## Setup

```bash
cd ness-bc
cp .env.template .env
# Edit .env with your values
cargo run
```

## Environment Variables

```bash
PRIVY_APP_ID=your_app_id
PRIVY_APP_SECRET=your_app_secret
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
RUST_LOG=info
```

## x402 Protocol Flow

```
1. Client requests resource
2. Server returns HTTP 402 + payment details
3. Client signs Solana transaction (via Privy wallet)
4. Client retries with X-Payment header
5. Server verifies payment, returns resource
```

### 402 Response

```json
{
  "error": {
    "code": "PAYMENT_REQUIRED",
    "message": "Payment required to access resource"
  },
  "payment": {
    "amount": 0.001,
    "currency": "SOL",
    "recipient": "recipient_wallet_address",
    "memo": "resource_id"
  }
}
```

### Payment Header

```
X-Payment: <base64_encoded_signed_transaction>
```

## Privy Server-Side Operations

### Create Wallet for User

```rust
// POST https://api.privy.io/v1/wallets
// owner: { type: "user", id: "privy_user_did" }
// chain_type: "solana"
```

### Sign Transaction (Server Wallet)

```rust
// For server-owned wallets only
// User wallets sign via iOS app with biometric MFA
```

## Project Structure

```
src/
├── main.rs          # Entry point
├── lib.rs           # Library exports
├── privy/           # Privy API client
├── solana/          # Solana operations
└── x402/            # x402 protocol
```

## Docs

- [Solana Rust SDK](https://solana.com/docs/clients/official/rust)
- [Privy Rust](https://docs.privy.io/basics/rust/quickstart)
- [x402 Protocol](https://solana.com/x402/what-is-x402)
- [Privy Create Wallet](https://docs.privy.io/wallets/wallets/create/create-a-wallet)
