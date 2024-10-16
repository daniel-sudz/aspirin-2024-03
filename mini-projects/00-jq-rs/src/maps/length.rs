use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct LengthMap {
}

impl Map for LengthMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        match input == "length" {
            true => Ok(Box::new(LengthMap {})),
            false => anyhow::bail!("failed to parse length"),
        }
    }
}

