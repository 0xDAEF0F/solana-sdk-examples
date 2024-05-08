use anyhow::Result;
use bip39::Mnemonic;
use solana_sdk::{
    derivation_path::DerivationPath,
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};
use std::str::FromStr;

fn main() -> Result<()> {
    dotenv::dotenv()?;

    let mnemonic = std::env::var("MNEMONIC")?;
    let mnemonic = Mnemonic::from_str(&mnemonic)?;

    let seed = mnemonic.to_seed("");

    let dp = DerivationPath::new_bip44(Some(0), Some(0));

    let keypair = Keypair::from_seed_and_derivation_path(&seed, Some(dp)).unwrap();

    let pubkey = keypair.pubkey();

    println!("the pubkey is {:?}", pubkey);

    Ok(())
}
