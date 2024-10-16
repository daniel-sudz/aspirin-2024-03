use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct AddMap {
}

impl Map for AddMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        match input == "add" {
            true => Ok(Box::new(AddMap {})),
            false => anyhow::bail!("failed to parse add"),
        }
    }
}

