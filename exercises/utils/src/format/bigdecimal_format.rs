use std::str::FromStr;

use bigdecimal::BigDecimal;
use serde::{self, Deserialize, Deserializer, Serializer};

// const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn serialize<S>(num: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&num.to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    BigDecimal::from_str(&s).map_err(serde::de::Error::custom)
}
