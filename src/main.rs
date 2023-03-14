use std::str::FromStr;
use clap::{arg, Parser};

use solana_sdk::pubkey::Pubkey;

use crate::connections::clientconnection::build_client;

use crate::workers::signatures::get_signatures;
use crate::workers::tokens::get_token;

use crate::shared::structs::EarliestOutput;

pub mod connections;
pub mod workers;
pub mod shared;

#[derive(Parser)]
/// The beginnings of a forensics toolkit for the Solana blockchain. This runs against mainnet-beta.
struct Args {
   /// Wallet address
   #[arg(short, long)]
   addr: String,

   /// Whether the address is a Token account
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

    let pubkey = addr.to_string();

    // Passing the RpcClient and Pubkey into the function
    let earliest_signature = get_signatures(&rpc, addr)?;

    // Initialize the mutable variables outside if block
    let mut first_date = String::new();
    let mut first_tx = String::new();

    // Check the boolean args earliest.
    // Default is true
    if args.earliest {
        first_date = earliest_signature.0.to_string();
        first_tx = earliest_signature.1.to_string();
    }

    // Initialize the mutable variables outside if block
    let mut mint = None;
    let mut owner = None;

    // Check the boolean args token.
    // Default is false needs --token
    if args.token {
        let token = get_token(&rpc, &addr);
        mint = token.0;
        owner = token.1;
    }

    // Defining the output via the struct
    let output = EarliestOutput {
        pubkey,
        first_date,
        first_tx,
        mint,
        owner,
    };

    let output_json = serde_json::to_string_pretty(&output).unwrap();

    println!("{}", output_json);

    Ok(())

}