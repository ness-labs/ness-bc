use crate::{BcError, Result};
use reqwest::Client;
use super::types::*;

const PRIVY_API_URL: &str = "https://api.privy.io/v1";

pub struct PrivyClient {
    client: Client,
    app_id: String,
    app_secret: String,
}

impl PrivyClient {
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self {
            client: Client::new(),
            app_id,
            app_secret,
        }
    }

    pub fn from_env() -> Result<Self> {
        let app_id = std::env::var("PRIVY_APP_ID")
            .map_err(|_| BcError::PrivyError("PRIVY_APP_ID not set".into()))?;
        let app_secret = std::env::var("PRIVY_APP_SECRET")
            .map_err(|_| BcError::PrivyError("PRIVY_APP_SECRET not set".into()))?;
        Ok(Self::new(app_id, app_secret))
    }

    fn auth_header(&self) -> String {
        use base64::Engine;
        let credentials = format!("{}:{}", self.app_id, self.app_secret);
        format!("Basic {}", base64::engine::general_purpose::STANDARD.encode(credentials))
    }

    /// Get user by Privy DID
    pub async fn get_user(&self, user_id: &str) -> Result<PrivyUser> {
        let resp = self.client
            .get(format!("{}/users/{}", PRIVY_API_URL, user_id))
            .header("Authorization", self.auth_header())
            .header("privy-app-id", &self.app_id)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(BcError::PrivyError(format!("{}: {}", status, body)));
        }

        Ok(resp.json().await?)
    }

    /// Create a Solana wallet for user
    pub async fn create_wallet(&self, user_id: &str) -> Result<PrivyWallet> {
        let req = CreateWalletRequest {
            chain_type: "solana".into(),
            owner: WalletOwner {
                owner_type: "user".into(),
                id: user_id.into(),
            },
        };

        let resp = self.client
            .post(format!("{}/wallets", PRIVY_API_URL))
            .header("Authorization", self.auth_header())
            .header("privy-app-id", &self.app_id)
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(BcError::PrivyError(format!("{}: {}", status, body)));
        }

        Ok(resp.json().await?)
    }

    /// Get user's wallets
    pub async fn get_wallets(&self, user_id: &str) -> Result<Vec<PrivyWallet>> {
        let resp = self.client
            .get(format!("{}/users/{}/wallets", PRIVY_API_URL, user_id))
            .header("Authorization", self.auth_header())
            .header("privy-app-id", &self.app_id)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(BcError::PrivyError(format!("{}: {}", status, body)));
        }

        Ok(resp.json().await?)
    }

    /// Sign transaction with wallet
    pub async fn sign_transaction(
        &self,
        wallet_id: &str,
        transaction_base64: &str,
    ) -> Result<String> {
        let req = SignTransactionRequest {
            transaction: transaction_base64.into(),
        };

        let resp = self.client
            .post(format!("{}/wallets/{}/sign", PRIVY_API_URL, wallet_id))
            .header("Authorization", self.auth_header())
            .header("privy-app-id", &self.app_id)
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(BcError::PrivyError(format!("{}: {}", status, body)));
        }

        let result: SignTransactionResponse = resp.json().await?;
        Ok(result.signature)
    }
}
