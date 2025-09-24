use dotenvy::dotenv;
use once_cell::sync::Lazy;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
};
use std::{env, sync::Arc};
// pub const PUMP_FUN_PROGRAM_ID: Pubkey =
//     Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
// pub const RAYDIUM_LAUNCHPAD_PROGRAM_ID: Pubkey =
//     Pubkey::from_str_const("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");
// pub const MOONSHOT_PROGRAM_ID: Pubkey =
//     Pubkey::from_str_const("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG");
pub const SPL_TOKEN_PROGRAM: Pubkey =
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const META_PLEX_METADATA: Pubkey =
    Pubkey::from_str_const("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub const TRACKING_WALLET_ADDRESS: Pubkey =
    Pubkey::from_str_const("62YEopiDLQeMxKV7JDLp4F2GZ8Y8hY2VrA4AVDFGTWLW");

