use anyhow::Result;
use serde::Deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const STAN_SOL_ACCT_V4: &str = "FdaWCghYqyua2awm1bb2MLZCRUde3FLWkywb2Duz4vtg";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    let mem = std::mem::size_of::<StateLayoutV4>();

    println!("size of StateLayoutV4: {}", mem);
    // let rpc_url = std::env::var("MAINNET_BETA_RPC_URL")?;
    // let rpc_client = RpcClient::new(rpc_url);

    // let account_pubkey = Pubkey::from_str(STAN_SOL_ACCT_V4)?;

    // let acct_data: &[u8] = &rpc_client.get_account_data(&account_pubkey).await?;

    // println!("account data: {:?}", acct_data.len());

    // let storage: StateLayoutV4 = bincode::deserialize(acct_data)?;

    // println!("{:#?}", storage);

    Ok(())
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct StateLayoutV4 {
    status: u64,
    nonce: u64,
    max_order: u64,
    depth: u64,
    base_decimal: u64,
    quote_decimal: u64,
    state: u64,
    reset_flag: u64,
    min_size: u64,
    vol_max_cut_ratio: u64,
    amount_wave_ratio: u64,
    base_lot_size: u64,
    quote_lot_size: u64,
    min_price_multiplier: u64,
    max_price_multiplier: u64,
    system_decimal_value: u64,
    min_separate_numerator: u64,
    min_separate_denominator: u64,
    trade_fee_numerator: u64,
    trade_fee_denominator: u64,
    pnl_numerator: u64,
    pnl_denominator: u64,
    swap_fee_numerator: u64,
    swap_fee_denominator: u64,
    base_need_take_pnl: u64,
    quote_need_take_pnl: u64,
    quote_total_pnl: u64,
    base_total_pnl: u64,
    pool_open_time: u64,
    punish_pc_amount: u64,
    punish_coin_amount: u64,
    orderbook_to_init_time: u64,
    swap_base_in_amount: u128,
    swap_quote_out_amount: u128,
    swap_base_2_quote_fee: u64,
    swap_quote_in_amount: u128,
    swap_base_out_amount: u128,
    swap_quote_2_base_fee: u64,
    // amm vault
    base_vault: Pubkey,
    quote_vault: Pubkey,
    // mint
    base_mint: Pubkey,
    quote_mint: Pubkey,
    lp_mint: Pubkey,
    // market
    open_orders: Pubkey,
    market_id: Pubkey,
    market_program_id: Pubkey,
    target_orders: Pubkey,
    withdraw_queue: Pubkey,
    lp_vault: Pubkey,
    owner: Pubkey,
    // true circulating supply without lock up
    lp_reserve: u64,
    padding: [u64; 3],
}
