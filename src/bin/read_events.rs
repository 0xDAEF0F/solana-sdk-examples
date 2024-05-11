use anchor_client::Client;
use anchor_lang::prelude::*;
use anyhow::Result;
use solana_sdk_examples::{get_keypair, Increment};
use std::rc::Rc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let keypair = Rc::new(get_keypair()?);
    let client = Client::new(
        anchor_client::Cluster::Custom(
            "http://localhost:8899".to_string(),
            "ws://127.0.0.1:8900".to_string(),
        ),
        Rc::clone(&keypair),
    );
    let pid: Pubkey = "2rCfPNFf5jiHoYT76fedtiF7cAXj8hC72SxZPTJQjPKN".parse()?;

    let program = client.program(pid)?;

    let (sender, mut receiver) = mpsc::unbounded_channel();
    let event_unsubscriber = program
        .on(move |_, evt: Increment| {
            if sender.send(evt).is_err() {
                println!("error while transferring event");
            }
        })
        .await?;

    loop {
        if let Ok(evt) = receiver.try_recv() {
            println!("{:#?}", evt);
            event_unsubscriber.unsubscribe().await;
            break;
        }
    }

    Ok(())
}
