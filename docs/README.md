# ness-bc - Blockchain Integration

Solana, Privy, and x402 integration for the Ness payment platform.

## Overview

Provides:
- Privy API client for wallet management
- Solana RPC client for balance/transaction queries
- x402 payment protocol handler

## Modules

### Privy Client

Manages user wallets via Privy's server-side API:

```rust
use ness_bc::privy::PrivyClient;

let privy = PrivyClient::from_env()?;

// Get user info
let user = privy.get_user("did:privy:123").await?;

// Create Solana wallet for user
let wallet = privy.create_wallet("did:privy:123").await?;

// Get user's wallets
let wallets = privy.get_wallets("did:privy:123").await?;

// Sign transaction (server-side)
let signature = privy.sign_transaction(
    "wallet_id",
    "base64_encoded_transaction"
).await?;
```

### Solana Client

Interacts with Solana blockchain:

```rust
use ness_bc::solana::SolanaClient;

let solana = SolanaClient::from_env()?;

// Get wallet balance
let balance = solana.get_balance("wallet_address")?;
println!("{} SOL", balance.sol);

// Confirm transaction
let result = solana.confirm_transaction("signature")?;
if result.confirmed {
    println!("Transaction confirmed at slot {}", result.slot);
}

// Check sufficient balance
solana.check_balance("address", 50_000_000)?; // 0.05 SOL
```

### x402 Handler

Implements the x402 payment protocol:

```rust
use ness_bc::x402::{X402Handler, ServicePricing};

let x402 = X402Handler::from_env()?;

// Create payment required response
let pricing = ServicePricing::video_generation();
let payment_info = x402.create_payment_required(&pricing);

// Returns:
// {
//   "payment_address": "...",
//   "amount_lamports": 50000000,
//   "amount_sol": 0.05,
//   "currency": "SOL",
//   "description": "Generate an 8-second video using Gemini Veo3",
//   "resource_id": "veo3-video-8s:uuid",
//   "expires_at": 1734267600
// }

// Verify payment proof
let verification = x402.verify_payment(&proof)?;
if verification.valid {
    // Grant access to resource
}
```

## x402 Protocol Flow

```
1. Client requests resource
   GET /api/video/generate

2. Server returns 402 Payment Required
   {
     "payment_address": "...",
     "amount_sol": 0.05,
     "resource_id": "veo3:abc123"
   }

3. Client signs Solana transaction via Privy
   (triggers biometric MFA on device)

4. Client retries with payment proof
   GET /api/video/generate
   X-Payment-Proof: {"transaction_signature": "...", ...}

5. Server verifies payment on Solana
   - Confirms transaction
   - Validates amount and recipient
   - Checks idempotency

6. Server returns resource
   200 OK with video data
```

## Service Pricing

Pre-configured services:

| Service ID | Price | Description |
|------------|-------|-------------|
| `veo3-video-8s` | 0.05 SOL | 8-second AI video generation |

Add custom pricing:

```rust
let custom = ServicePricing {
    service_id: "custom-service".into(),
    name: "Custom Service".into(),
    price_lamports: 10_000_000, // 0.01 SOL
    description: "Description".into(),
};
```

## Project Structure

```
src/
├── lib.rs           # Library exports
├── main.rs          # CLI for testing
├── error.rs         # Error types
├── privy/
│   ├── mod.rs
│   ├── client.rs    # Privy API client
│   └── types.rs     # Privy data types
├── solana/
│   ├── mod.rs
│   ├── client.rs    # Solana RPC client
│   └── types.rs     # Solana data types
└── x402/
    ├── mod.rs
    ├── handler.rs   # x402 payment handler
    └── types.rs     # x402 data types
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `PRIVY_APP_ID` | Yes | Privy application ID |
| `PRIVY_APP_SECRET` | Yes | Privy application secret |
| `SOLANA_RPC_URL` | No | Solana RPC (default: devnet) |
| `NESS_PAYMENT_ADDRESS` | Yes | Wallet for receiving payments |

## Security Notes

- Never log or store private keys
- Privy handles all key custody
- All wallet operations require user MFA
- Verify transactions on-chain before granting access
