use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;
pub struct IdentityMap;

impl Map for IdentityMap {
    fn map(&self, value: Result<Value>) -> Result<Value> {
        value
    }
}

