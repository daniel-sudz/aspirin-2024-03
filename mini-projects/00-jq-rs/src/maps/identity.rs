use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;
pub struct IdentityMap;

impl Map for IdentityMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        match input == "." {
            true => Ok(Box::new(IdentityMap)),
            false => anyhow::bail!("failed to parse identity"),
        }
    }
}

