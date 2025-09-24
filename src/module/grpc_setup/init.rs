use futures::{SinkExt, StreamExt};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{pubkey::Pubkey};

use std::{collections::HashMap};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::{
    geyser::{
        CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions, SubscribeUpdate,
        subscribe_update::UpdateOneof,
    },
    prelude::CompiledInstruction,
    tonic::Status,
};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct DataV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Pubkey>, // simplified: you can use the real Collection struct if needed
    pub uses: Option<u8>,           // simplified
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CreateMetadataAccountV3Ix {
    pub discriminator: u8,
    pub data: DataV2,
    pub is_mutable: bool,
    pub collection_details: Option<u8>, // simplified
}

fn parse_metadata_ix_data(data: &[u8]) -> Option<CreateMetadataAccountV3Ix> {
    let mut slice = data;
    CreateMetadataAccountV3Ix::deserialize(&mut slice).ok()
}


pub async fn setup_client_grpc(
    grpc_endpoint: String, // Accepts &str directly, no need for &String
    x_token: String,       // Same here, accepts &str directly
) -> Result<GeyserGrpcClient<impl Interceptor>, Box<dyn std::error::Error>> {
    // Build the gRPC client with TLS config
    let client = GeyserGrpcClient::build_from_shared(grpc_endpoint.to_string())?
        .x_token(Some(x_token.to_string()))?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;

    Ok(client)
}
/// Send the subscription request with transaction filters
pub async fn send_subscription_request_grpc<T>(
    mut tx: T,
    subscribe_args: SubscribeRequestFilterTransactions,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: SinkExt<SubscribeRequest> + Unpin,
    <T as futures::Sink<SubscribeRequest>>::Error: std::error::Error + 'static,
{
    // Create account filter with the target accounts
    let mut accounts_filter = HashMap::new();
    accounts_filter.insert("account_monitor".to_string(), subscribe_args);

    // Send subscription request
    tx.send(SubscribeRequest {
        transactions: accounts_filter,
        commitment: Some(CommitmentLevel::Processed as i32),
        ..Default::default()
    })
    .await?;

    Ok(())
}

pub async fn process_updates_grpc<S>(mut stream: S) -> Result<(), Box<dyn std::error::Error>>
where
    S: StreamExt<Item = Result<SubscribeUpdate, Status>> + Unpin,
{
    while let Some(result) = stream.next().await {
        match result {
            Ok(update) => {
                let (account_keys, ixs, tx_id) =
                    if let Some(data) = extract_transaction_data(&update) {
                        data
                    } else {
                        continue;
                    };
                trade_info(ixs, account_keys);
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
            }
        }
    }

    Ok(())
}

pub fn extract_transaction_data(
    update: &SubscribeUpdate,
) -> Option<(Vec<Pubkey>, Vec<CompiledInstruction>, String)> {
    // borrow the enum inside update_oneof to avoid move
    let transaction_update = match &update.update_oneof {
        Some(UpdateOneof::Transaction(tx_update)) => tx_update,
        _ => return None,
    };

    // safely get references to nested fields
    let tx_info = transaction_update.transaction.as_ref()?;
    let transaction = tx_info.transaction.as_ref()?;
    let meta = tx_info.meta.as_ref()?;
    let tx_msg = transaction.message.as_ref()?;

    // Assume you already have this:
    let mut account_keys: Vec<Pubkey> = tx_msg
        .account_keys
        .iter()
        .filter_map(|k| Pubkey::try_from(k.as_slice()).ok())
        .collect();

    // Append loaded_writable_addresses
    account_keys.extend(
        meta.loaded_writable_addresses
            .iter()
            .filter_map(|raw| Pubkey::try_from(raw.as_slice()).ok()),
    );

    // Append loaded_readonly_addresses
    account_keys.extend(
        meta.loaded_readonly_addresses
            .iter()
            .filter_map(|raw| Pubkey::try_from(raw.as_slice()).ok()),
    );

    let ixs: Vec<CompiledInstruction> = tx_msg.instructions.clone();

    let signature = &tx_info.signature;
    let tx_id = bs58::encode(signature).into_string();

    Some((account_keys, ixs, tx_id))
}

pub fn trade_info(ixs: Vec<CompiledInstruction>, account_keys: Vec<Pubkey>) {
    for ix in ixs {
        // check program id if needed:
        // if account_keys[ix.program_id_index as usize] != YOUR_PROGRAM_ID { continue; }

        if let Some(parsed) = parse_metadata_ix_data(&ix.data) {
            let log_json = serde_json::json!({
                "discriminator": { "type": "u8", "data": parsed.discriminator },
                "dataV2": {
                    "name": parsed.data.name,
                    "symbol": parsed.data.symbol,
                    "uri": parsed.data.uri,
                    "sellerFeeBasisPoints": parsed.data.seller_fee_basis_points,
                    "creators": parsed.data.creators.map(|c| c.iter().map(|cr| {
                        serde_json::json!({
                            "address": cr.address.to_string(),
                            "verified": cr.verified,
                            "share": cr.share
                        })
                    }).collect::<Vec<_>>()),
                    "collection": parsed.data.collection,
                    "uses": parsed.data.uses
                },
                "isMutable": { "type": "u8", "data": parsed.is_mutable },
                "collectionDetails": parsed.collection_details,
            });

            println!("{}", serde_json::to_string_pretty(&log_json).unwrap());
        }
    }
}
