use anchor_lang::prelude::*;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::address_lookup_table::program::ID;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    // instantiate rpc client
    let rpc_url = std::env::var("RPC_URL")?;
    let rpc_client = RpcClient::new(rpc_url);

    // you must have an acct deployed first and change this pubkey
    let account_pubkey = Pubkey::from_str("43HffmHLr6nRWwQnhAKDhqjWp4DUm52vFrSP1nA6JV4r")?;

    let mut acct_data: &[u8] = &rpc_client.get_account_data(&account_pubkey).await?;

    let storage = Storage::try_deserialize(&mut acct_data)?;

    println!("the storage is {:#?}", storage);

    Ok(())
}

#[account]
#[derive(Debug)]
pub struct Storage {
    counter: u64,
}
