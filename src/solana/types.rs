use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    pub address: String,
    pub lamports: u64,
    pub sol: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub signature: String,
    pub slot: u64,
    pub confirmed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from_address: String,
    pub to_address: String,
    pub lamports: u64,
}
