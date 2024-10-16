use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;
use regex::Regex;
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

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"\.(\w+)";
        let re: Regex = Regex::new(pattern).unwrap();
    
        match re.captures(input) {
            Some(captures) => {
                let key = captures.get(1).unwrap().as_str();
                return Ok(Box::new(ObjectIdentifierMap { key: key.to_string() }))
            },
            None => anyhow::bail!("failed to parse object identifier"),
        }
    }
}
