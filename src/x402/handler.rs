use crate::{BcError, Result, solana::SolanaClient};
use super::types::*;
use chrono::Utc;

pub struct X402Handler {
    solana: SolanaClient,
    payment_address: String,
}

impl X402Handler {
    pub fn new(solana: SolanaClient, payment_address: String) -> Self {
        Self { solana, payment_address }
    }

    pub fn from_env() -> Result<Self> {
        let solana = SolanaClient::from_env()?;
        let payment_address = std::env::var("NESS_PAYMENT_ADDRESS")
            .map_err(|_| BcError::X402Error("NESS_PAYMENT_ADDRESS not set".into()))?;
        Ok(Self::new(solana, payment_address))
    }

    /// Create a 402 Payment Required response
    pub fn create_payment_required(&self, service: &ServicePricing) -> X402PaymentRequired {
        let expires_at = Utc::now().timestamp() + 300; // 5 minutes

        X402PaymentRequired {
            payment_address: self.payment_address.clone(),
            amount_lamports: service.price_lamports,
            amount_sol: service.price_lamports as f64 / 1_000_000_000.0,
            currency: "SOL".into(),
            description: service.description.clone(),
            resource_id: format!("{}:{}", service.service_id, uuid::Uuid::new_v4()),
            expires_at,
        }
    }

    /// Verify payment proof
    pub fn verify_payment(&self, proof: &X402PaymentProof) -> Result<PaymentVerification> {
        // Confirm transaction on Solana
        let tx_result = self.solana.confirm_transaction(&proof.transaction_signature)?;

        if !tx_result.confirmed {
            return Err(BcError::X402Error("Transaction not confirmed".into()));
        }

        // In production, you'd verify:
        // 1. Transaction recipient matches payment_address
        // 2. Amount matches required amount
        // 3. Transaction is recent (not replayed)

        Ok(PaymentVerification {
            valid: true,
            resource_id: proof.resource_id.clone(),
            amount_paid: 0, // Would extract from transaction
            payer: proof.payer_address.clone(),
            signature: proof.transaction_signature.clone(),
        })
    }

    /// Check if request has valid payment
    pub fn check_payment_header(&self, header: Option<&str>) -> Result<PaymentVerification> {
        let header = header.ok_or_else(|| BcError::X402Error("Missing X-Payment-Proof header".into()))?;

        let proof: X402PaymentProof = serde_json::from_str(header)
            .map_err(|e| BcError::X402Error(format!("Invalid payment proof: {}", e)))?;

        self.verify_payment(&proof)
    }
}
