use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

use crate::shared::utilities::convert_epoch;

pub fn get_signatures(rpc: &RpcClient, addr: Pubkey) -> Result<(String, String), Box<dyn std::error::Error>> {

    fn fetch_more_signatures(rpc: &RpcClient, addr: &Pubkey, before: Option<Signature>, ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let signatures = rpc.get_signatures_for_address_with_config(
            &addr,
            solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config {
                before,
                ..Default::default()
            },
        )?;

        let earliest = signatures.clone().into_iter().next_back().unwrap();

        if signatures.len() < 1000 {

            let datetime_utc = convert_epoch(earliest.block_time.unwrap());

            let sig_tup = (datetime_utc.unwrap().to_string(), earliest.signature.to_string());

            Ok(sig_tup)

        } else {
            let signature = Signature::from_str(&earliest.signature)?;

            fetch_more_signatures(&rpc, &addr, Some(signature))
        }
    }

    let sig_tup = fetch_more_signatures(&rpc, &addr, None)?;

    Ok(sig_tup)
}