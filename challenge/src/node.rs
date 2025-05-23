use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "publicKey")]
    pub pub_key: String,
    pub alias: String,

    #[serde(rename = "capacity")]
    #[serde(serialize_with = "s_capacity")]
    pub capacity: u64,

    #[serde(rename = "firstSeen")]
    #[serde(serialize_with = "s_timestamp")]
    pub first_seen: i64,
}

fn s_capacity<S>(capacity: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_f64(*capacity as f64 / 100_000_000.0)
}

fn s_timestamp<S>(timestamp: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let datetime = Utc.timestamp_opt(*timestamp, 0).unwrap();
    serializer.serialize_str(&datetime.to_rfc3339())
}
