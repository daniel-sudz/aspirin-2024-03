use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct LengthMap {
}

impl Map for LengthMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }
}

