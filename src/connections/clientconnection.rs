//use std::str::FromStr;
use solana_client::rpc_client::RpcClient;

pub fn build_client() -> RpcClient {
    let rpc = RpcClient::new(String::from("https://api.mainnet-beta.solana.com"));

    rpc
}