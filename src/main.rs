use std::str::FromStr;
use clap::{arg, Parser};

use solana_sdk::pubkey::Pubkey;

use crate::connections::clientconnection::build_client;

use crate::workers::signatures::get_signatures;
//use crate::workers::tx_details::get_transaction_details;
use crate::workers::tokens::get_token;

pub mod utils;
pub mod connections;
pub mod workers;

#[derive(Parser)]
/// The beginnings of a forensics toolkit for the Solana blockchain. This runs against mainnet-beta.
struct Args {
   /// Wallet address
   #[arg(short, long)]
   addr: String,

    /// Whether or not the address is a Token account
   #[arg(short, long, default_value_t = false)]
   token: bool,

   /// Earliest transaction signature and its respective date
   #[arg(short, long, default_value_t = true)]
   earliest: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    // New RpcClient connection with Mainnet Beta
    let rpc = build_client();

    let addr = Pubkey::from_str(&args.addr.to_string()).unwrap();

    println!("Pubkey: {}", addr.to_string());

    // Passing the RpcClient and Pubkey into the function
    let earliest_signature = get_signatures(&rpc, addr)?;

    // Check the boolean args earliest.
    if args.earliest {
        println!("\tFirst: {} \n\tTx: {}", earliest_signature.1.to_string(), earliest_signature.0.to_string());
    } else {
        println!("Only support earliest.")
    }

    // Calling get_token
    if args.token {
        let token = get_token(&rpc, &addr);
        println!("\tMint: {} \n\tOwner: {}", &token.0, &token.1);
    }

    /*
    let sig = Signature::from_str(&earliest_signature.0).unwrap();

    // Get the transaction details for the respective signature
    get_transaction_details(sig, &rpc);
    */

    Ok(())

}