use serde::{Deserialize, Serialize};

/// x402 Payment Required response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402PaymentRequired {
    pub payment_address: String,
    pub amount_lamports: u64,
    pub amount_sol: f64,
    pub currency: String,
    pub description: String,
    pub resource_id: String,
    pub expires_at: i64,
}

/// Payment proof header sent by client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402PaymentProof {
    pub transaction_signature: String,
    pub payer_address: String,
    pub resource_id: String,
}

/// Payment verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentVerification {
    pub valid: bool,
    pub resource_id: String,
    pub amount_paid: u64,
    pub payer: String,
    pub signature: String,
}

/// Service pricing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePricing {
    pub service_id: String,
    pub name: String,
    pub price_lamports: u64,
    pub description: String,
}

impl ServicePricing {
    pub fn video_generation() -> Self {
        Self {
            service_id: "veo3-video-8s".into(),
            name: "Veo3 8-second Video Generation".into(),
            price_lamports: 50_000_000, // 0.05 SOL
            description: "Generate an 8-second video using Gemini Veo3".into(),
        }
    }
}
