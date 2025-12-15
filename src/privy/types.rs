use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivyUser {
    pub id: String, // Privy DID - stable identifier
    pub created_at: i64,
    #[serde(default)]
    pub linked_accounts: Vec<LinkedAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LinkedAccount {
    #[serde(rename = "email")]
    Email { address: String },
    #[serde(rename = "wallet")]
    Wallet {
        address: String,
        chain_type: String,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivyWallet {
    pub id: String,
    pub address: String,
    pub chain_type: String,
}

#[derive(Debug, Serialize)]
pub struct CreateWalletRequest {
    pub chain_type: String,
    pub owner: WalletOwner,
}

#[derive(Debug, Serialize)]
pub struct WalletOwner {
    #[serde(rename = "type")]
    pub owner_type: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct SignTransactionRequest {
    pub transaction: String, // base64 encoded
}

#[derive(Debug, Deserialize)]
pub struct SignTransactionResponse {
    pub signature: String,
}
