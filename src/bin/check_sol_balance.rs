use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signer::Signer;
use solana_sdk_examples::get_keypair;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let rpc_url = std::env::var("RPC_URL")?;
    let rpc_client = RpcClient::new(rpc_url);

    let keypair = get_keypair()?;
    let pubkey = keypair.pubkey();

    let balance = rpc_client.get_balance(&pubkey).await?;

    println!("pubkey: {} - balance: {}", pubkey, balance);

    Ok(())
}
