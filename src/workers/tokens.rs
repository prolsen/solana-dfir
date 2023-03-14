use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub fn get_token(rpc: &RpcClient, addr: &Pubkey) -> (Option<String>, Option<String>) {
    let token_result = rpc.get_token_account(addr);
    let token =
        match token_result {
            Ok(token) => (Some(token.as_ref().unwrap().mint.to_string()),
                                                Some(token.as_ref().unwrap().owner.to_string())),
            Err(_err) => {
                (None, None)
            }
        };

    token
}
