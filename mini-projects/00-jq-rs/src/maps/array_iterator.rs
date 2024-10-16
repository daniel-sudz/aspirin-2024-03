use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct ArrayIteratorMap {
    pub from: usize,
    pub to: usize,
}

impl Map for ArrayIteratorMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        let result: Result<Vec<Value>> = value.iter().map(|v| {
            let array = v.as_array().ok_or_else(|| anyhow::anyhow!("array iterator requires an array"))?;
            let slice = array[self.from..self.to].to_vec();
            Ok(Value::Array(slice))
        }).collect();
        result
    }
}

