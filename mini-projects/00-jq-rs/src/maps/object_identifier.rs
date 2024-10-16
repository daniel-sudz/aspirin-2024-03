use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct ObjectIdentifierMap {
    pub key: String,
}

impl Map for ObjectIdentifierMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        let result: Result<Vec<Value>> = value.iter().map(|v| {
            let new_value = v[&self.key].clone();
            if new_value.is_null() {
                anyhow::bail!("object identifier not found")
            }
            Ok(new_value)
        }).collect();
        result
    }
}

