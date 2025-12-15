use thiserror::Error;

#[derive(Error, Debug)]
pub enum BcError {
    #[error("Privy API error: {0}")]
    PrivyError(String),

    #[error("Solana RPC error: {0}")]
    SolanaError(String),

    #[error("x402 payment error: {0}")]
    X402Error(String),

    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: f64, available: f64 },

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, BcError>;
