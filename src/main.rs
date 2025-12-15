use ness_bc::{privy::PrivyClient, solana::SolanaClient, x402::X402Handler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    println!("ness-bc: Blockchain integration module");
    println!("- Privy wallet management");
    println!("- Solana transactions");
    println!("- x402 payment protocol");

    // Example: Initialize clients
    if let Ok(solana) = SolanaClient::from_env() {
        println!("✓ Solana client initialized");
        
        // Test connection
        if let Ok(blockhash) = solana.get_recent_blockhash() {
            println!("  Latest blockhash: {}...", &blockhash[..16]);
        }
    }

    if let Ok(_privy) = PrivyClient::from_env() {
        println!("✓ Privy client initialized");
    }

    if let Ok(_x402) = X402Handler::from_env() {
        println!("✓ x402 handler initialized");
    }

    Ok(())
}
