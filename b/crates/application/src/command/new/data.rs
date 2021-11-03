use serde_json::Value;
use std::{
    collections::BTreeMap,
    io::{self, Read},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildDataError {
    #[error("read error")]
    Read(#[from] io::Error),
    #[error("format error")]
    Format(#[from] serde_json::Error),
    #[error("root object error")]
    RootObject,
    #[error("invalid key error")]
    InvalidKey,
    #[error("invalid value error")]
    InvalidValue,
}

pub fn build_data(handle: &mut impl Read) -> Result<BTreeMap<String, String>, BuildDataError> {
    let mut data = String::new();
    handle.read_to_string(&mut data)?;
    let data: Value = serde_json::from_str(&data)?;
    let object = data.as_object().ok_or(BuildDataError::RootObject)?;
    let mut map = BTreeMap::new();
    for (k, v) in object {
        if !k.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(BuildDataError::InvalidKey);
        }
        let v = v.as_str().ok_or(BuildDataError::InvalidValue)?.to_string();
        map.insert(k.clone(), v);
    }
    Ok(map)
}
