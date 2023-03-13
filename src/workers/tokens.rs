use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_account_decoder::parse_token::UiTokenAccount;

pub fn get_token(rpc: &RpcClient, addr: &Pubkey) -> (String, String) {

    let token: Option<UiTokenAccount> = rpc.get_token_account(addr).unwrap();

    (
        token.as_ref().unwrap().mint.to_string(),
        token.as_ref().unwrap().owner.to_string()
    )

}
