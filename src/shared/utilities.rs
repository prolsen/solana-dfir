use chrono::{DateTime, LocalResult, TimeZone, Utc};
use solana_sdk::clock::UnixTimestamp;

pub(crate) fn _convert_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

pub(crate) fn convert_epoch(time: UnixTimestamp) -> LocalResult<DateTime<Utc>> {
    let dt = Utc.timestamp_opt(time, 0);

    dt

}