#![allow(unused_imports)]
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer, system_instruction,
    system_transaction, transaction::Transaction,
};
use solana_sdk_examples::get_keypair;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    // instantiate rpc client
    let rpc_url = std::env::var("RPC_URL")?;
    let rpc_client = RpcClient::new(rpc_url);

    // generate sender and recipient keypairs
    let sender_keypair = get_keypair()?;
    let sender_pubkey = sender_keypair.pubkey();

    let recipient = Keypair::new();

    // print both balances before
    let sender_balance = rpc_client.get_balance(&sender_pubkey).await?;
    let recipient_balance = rpc_client.get_balance(&recipient.pubkey()).await?;
    println!(
        "BEFORE - sender: {:?} - balance: {} LAMPORTS",
        sender_pubkey, sender_balance
    );
    println!(
        "BEFORE - recipient: {:?} - balance: {} LAMPORTS",
        recipient.pubkey(),
        recipient_balance
    );

    // make the transaction
    // 0.1 SOL (make sure to first airdrop the acct)
    let lamports = LAMPORTS_PER_SOL / 10;
    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    // two instructions in one txn
    let ix = system_instruction::transfer(&sender_pubkey, &recipient.pubkey(), lamports);
    let ix2 = system_instruction::transfer(&recipient.pubkey(), &sender_pubkey, lamports);

    let tx = Transaction::new_signed_with_payer(
        &[ix, ix2],
        Some(&sender_keypair.pubkey()),
        &[&recipient, &sender_keypair], // they dont have to be in order
        recent_blockhash,
    );

    // this is if you want to not do an instruction individually
    // prepare txn
    // let tx = system_transaction::transfer(
    //     &sender_keypair,
    //     &recipient.pubkey(),
    //     lamports,
    //     recent_blockhash,
    // );

    // send txn
    let signature = rpc_client.send_and_confirm_transaction(&tx).await?;
    println!("txn signature: {:?}", signature);

    // print both balances after
    let sender_balance = rpc_client.get_balance(&sender_pubkey).await?;
    let recipient_balance = rpc_client.get_balance(&recipient.pubkey()).await?;
    println!(
        "AFTER - sender: {:?} - balance: {} LAMPORTS",
        sender_pubkey, sender_balance
    );
    println!(
        "AFTER - recipient: {:?} - balance: {} LAMPORTS",
        recipient.pubkey(),
        recipient_balance
    );

    Ok(())
}
