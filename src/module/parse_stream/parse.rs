use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::{
    geyser::{SubscribeUpdate, subscribe_update::UpdateOneof},
    prelude::CompiledInstruction,
}; // or use println! if you’re not using the `log` crate

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


