use anchor_lang::prelude::*;
use anyhow::Result;
use bip39::Mnemonic;
use solana_sdk::address_lookup_table::program::ID;
use solana_sdk::{derivation_path::DerivationPath, signature::Keypair, signer::SeedDerivable};
use std::str::FromStr;

pub fn get_keypair() -> Result<Keypair> {
    let mnemonic = std::env::var("MNEMONIC")?;
    let mnemonic = Mnemonic::from_str(&mnemonic)?;

    let seed = mnemonic.to_seed("");
    let dp = DerivationPath::new_bip44(Some(0), Some(0));

    // TODO: wrap the error message
    let keypair = Keypair::from_seed_and_derivation_path(&seed, Some(dp)).unwrap();

    Ok(keypair)
}

#[account]
#[derive(Debug)]
pub struct Storage {
    counter: u64,
}
