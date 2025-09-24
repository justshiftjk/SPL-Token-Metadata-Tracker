use axum::{Router, routing::get, routing::post};
use boris_wallet_tracker_grpc::*;
use hyper::server;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use yellowstone_grpc_proto::geyser::SubscribeRequestFilterTransactions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // ✅ Start GRPC subscription and processing
    let mut grpc_client = setup_client_grpc(GRPC_ENDPOINT.to_string(), GRPC_TOKEN.to_string())
        .await
        .expect("Failed to connect to GRPC");

    let (subscribe_tx, subscribe_rx) = grpc_client.subscribe().await.unwrap();

    let subscribe_filter = SubscribeRequestFilterTransactions {
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![
            TRACKING_WALLET_ADDRESS.to_string(),
            SPL_TOKEN_PROGRAM.to_string(),
            META_PLEX_METADATA.to_string(),
        ],
        vote: Some(false),
        failed: Some(false),
        signature: None,
    };

    send_subscription_request_grpc(subscribe_tx, subscribe_filter)
        .await
        .unwrap();

    if let Err(e) = process_updates_grpc(subscribe_rx).await {
        eprintln!("[GRPC] Error processing updates: {e:?}");
    };

    Ok(())
}
