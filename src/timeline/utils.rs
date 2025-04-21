use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

pub fn string_key_map<'de, D, V>(deserializer: D) -> Result<HashMap<u64, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de>,
{
    let string_map: HashMap<String, V> = HashMap::deserialize(deserializer)?;
    let mut int_map = HashMap::with_capacity(string_map.len());
    for (k, v) in string_map {
        let parsed_key = k.parse::<u64>().map_err(serde::de::Error::custom)?;
        int_map.insert(parsed_key, v);
    }
    Ok(int_map)
}
