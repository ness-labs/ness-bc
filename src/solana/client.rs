use crate::{BcError, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::Signature,
};
use std::str::FromStr;
use super::types::*;

pub struct SolanaClient {
    rpc: RpcClient,
}

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc: RpcClient::new_with_commitment(
                rpc_url.to_string(),
                CommitmentConfig::confirmed(),
            ),
        }
    }

    pub fn from_env() -> Result<Self> {
        let rpc_url = std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".into());
        Ok(Self::new(&rpc_url))
    }

    /// Get wallet balance
    pub fn get_balance(&self, address: &str) -> Result<WalletBalance> {
        let pubkey = Pubkey::from_str(address)
            .map_err(|e| BcError::SolanaError(format!("Invalid address: {}", e)))?;

        let lamports = self.rpc
            .get_balance(&pubkey)
            .map_err(|e| BcError::SolanaError(e.to_string()))?;

        Ok(WalletBalance {
            address: address.to_string(),
            lamports,
            sol: lamports as f64 / LAMPORTS_PER_SOL as f64,
        })
    }

    /// Confirm transaction
    pub fn confirm_transaction(&self, signature: &str) -> Result<TransactionResult> {
        let sig = Signature::from_str(signature)
            .map_err(|e| BcError::SolanaError(format!("Invalid signature: {}", e)))?;

        let confirmed = self.rpc
            .confirm_transaction(&sig)
            .map_err(|e| BcError::SolanaError(e.to_string()))?;

        let slot = self.rpc
            .get_slot()
            .map_err(|e| BcError::SolanaError(e.to_string()))?;

        Ok(TransactionResult {
            signature: signature.to_string(),
            slot,
            confirmed,
        })
    }

    /// Get recent blockhash for transaction building
    pub fn get_recent_blockhash(&self) -> Result<String> {
        let blockhash = self.rpc
            .get_latest_blockhash()
            .map_err(|e| BcError::SolanaError(e.to_string()))?;
        Ok(blockhash.to_string())
    }

    /// Check if address has sufficient balance
    pub fn check_balance(&self, address: &str, required_lamports: u64) -> Result<bool> {
        let balance = self.get_balance(address)?;
        if balance.lamports < required_lamports {
            return Err(BcError::InsufficientBalance {
                required: required_lamports as f64 / LAMPORTS_PER_SOL as f64,
                available: balance.sol,
            });
        }
        Ok(true)
    }
}
