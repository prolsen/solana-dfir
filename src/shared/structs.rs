use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EarliestOutput {
    pub pubkey: String,
    pub first_date: String,
    pub first_tx: String,
    pub mint: Option<String>,
    pub owner: Option<String>,
}