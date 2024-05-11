use anchor_lang::prelude::*;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signer::Signer, transaction::Transaction,
};
use solana_sdk_examples::{get_keypair, Storage};
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

    let ix = Instruction {
        data: vec![11, 18, 104, 9, 104, 174, 59, 33],
        program_id: Pubkey::from_str("2rCfPNFf5jiHoYT76fedtiF7cAXj8hC72SxZPTJQjPKN")?,
        accounts: vec![AccountMeta {
            pubkey: account_pubkey,
            is_signer: false,
            is_writable: true,
        }],
    };

    let recent_blockhash = rpc_client.get_latest_blockhash().await?;
    let sender_keypair = get_keypair()?;
    let mut transaction = Transaction::new_with_payer(&[ix], Some(&sender_keypair.pubkey()));
    transaction.sign(&[&sender_keypair], recent_blockhash);

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .await?;

    println!("transaction sent. signature: {}", signature);

    let mut acct_data: &[u8] = &rpc_client.get_account_data(&account_pubkey).await?;

    let storage = Storage::try_deserialize(&mut acct_data)?;

    println!("the storage now is {:#?}", storage);

    Ok(())
}
