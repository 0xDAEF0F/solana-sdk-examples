#![allow(unused_imports)]

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use solana_account_decoder::{UiAccountData, UiDataSliceConfig};
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

const STAN_SOL_ACCT_V4: &str = "FdaWCghYqyua2awm1bb2MLZCRUde3FLWkywb2Duz4vtg";
const HELIUS_WS: &str =
    "wss://atlas-mainnet.helius-rpc.com/?api-key=97fa217b-4825-4a4d-93a3-a915e3b927e8";

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse(HELIUS_WS)?;
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    // let (write, read) = ws_stream.split();

    let msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "accountSubscribe",
        "params": [
            STAN_SOL_ACCT_V4,
            {
                "encoding": "jsonParsed",
                "commitment": "confirmed"
            }
        ]
    });

    let msg = Message::Text(msg.to_string());
    ws_stream.send(msg).await?;

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(msg) => {
                println!("{:#?}", msg)
            }
            Err(e) => {
                eprintln!("Error receiving message: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
