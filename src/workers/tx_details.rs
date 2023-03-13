use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::{UiTransactionEncoding, EncodedConfirmedTransactionWithStatusMeta,
                                UiRawMessage, UiInnerInstructions, UiTransactionStatusMeta};
use solana_transaction_status::option_serializer::OptionSerializer;

use std::collections::BTreeMap;
use serde_json::Value;

pub(crate) fn get_transaction_details(signature: Signature, rpc: &RpcClient) {
    let transaction_details: EncodedConfirmedTransactionWithStatusMeta = rpc
        .get_transaction(&signature, UiTransactionEncoding::Json)
        .unwrap();

    // Ref: https://docs.rs/solana-transaction-status/1.15.2/solana_transaction_status/enum.EncodedTransaction.html

    let meta: UiTransactionStatusMeta = transaction_details.transaction.meta.unwrap();
    let meta_inner_instructions: OptionSerializer<Vec<UiInnerInstructions>> = meta.to_owned().inner_instructions;
    let _meta_pre_balances = meta.pre_balances;
    let _meta_post_balances = meta.post_balances;
    let _meta_pre_token_balances = meta.pre_token_balances;
    let _meta_post_token_balances = meta.post_token_balances;
    let _meta_log_messages: OptionSerializer<Vec<String>> = meta.log_messages;

    match meta_inner_instructions.clone() {
        OptionSerializer::Some(instructions) => {
            for ins in instructions.iter() {
                for x in &ins.instructions {
                    println!("{:?}", x);
                }
            }
        }
        OptionSerializer::None => {},
        OptionSerializer::Skip => {},
    }

    let tx = transaction_details.transaction.transaction;
    let tx_decoded_value = serde_json::to_value(tx).unwrap();
    let tx_object: Value = serde_json::from_str(&tx_decoded_value.to_string()).unwrap();

    // Extract the message header via object get.
    let tx_message = tx_object.get("message").unwrap().to_string();

    // Deserialize the UiRawMessage from a string (message)
    let tx_raw_msg: UiRawMessage = serde_json::from_str(&tx_message).unwrap();

    let mut act_key_map = BTreeMap::new();

    for (idx, val) in tx_raw_msg.account_keys.iter().enumerate() {
        act_key_map.insert(idx, val.to_string());
    };

    for x in tx_raw_msg.instructions.iter() {
        let key_id = x.program_id_index;
        let key_value = act_key_map.get(&(key_id as usize));
        println!("{} \t{}", key_id, key_value.unwrap().to_string())
    };
}