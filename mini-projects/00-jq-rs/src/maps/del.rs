use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct DelMap {
    pub key: String,
}

impl Map for DelMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }
}

