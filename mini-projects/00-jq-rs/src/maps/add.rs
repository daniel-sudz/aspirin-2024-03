use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct AddMap {
}

impl Map for AddMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }
}

