use dotenvy::dotenv;
use once_cell::sync::Lazy;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
};
use solana_commitment_config::{CommitmentConfig};
use std::{env, sync::Arc};

pub static PRIVATE_KEY: Lazy<Keypair> = Lazy::new(|| {
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let payer: Keypair = Keypair::from_base58_string(private_key.as_str());

    payer
});
pub static PUBKEY: Lazy<Pubkey> = Lazy::new(|| {
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");

    let payer: Keypair = Keypair::from_base58_string(private_key.as_str());

    payer.pubkey()
});

pub static RPC_ENDPOINT: Lazy<String> = Lazy::new(|| {
    dotenv().ok();

    let rpc_endpoint = env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT must be set");

    rpc_endpoint
});

pub static RPC_CLIENT: Lazy<Arc<RpcClient>> = Lazy::new(|| {
    dotenv().ok();

    let rpc_endpoint = env::var("RPC_ENDPOINT").expect("RPC_ENDPOINT must be set");

    Arc::new(RpcClient::new_with_commitment(
        rpc_endpoint,
        CommitmentConfig::processed(),
    ))
});

pub static GRPC_ENDPOINT: Lazy<String> = Lazy::new(|| {
    dotenv().ok();

    let grpc_endpoint = env::var("GRPC_ENDPOINT").expect("GRPC_ENDPOINT must be set");

    grpc_endpoint
});

pub static GRPC_TOKEN: Lazy<String> = Lazy::new(|| {
    dotenv().ok();

    let grpc_token = env::var("GRPC_TOKEN").expect("GRPC_TOKEN must be set");

    grpc_token
});


// pub static TRACKING_WALLET_ADDRESS: Lazy<String> = Lazy::new(|| {
//     dotenv().ok();

//     let tw = env::var("TRACKING_WALLET").expect("TRACKING_WALLET must be set");

//     tw
// });


