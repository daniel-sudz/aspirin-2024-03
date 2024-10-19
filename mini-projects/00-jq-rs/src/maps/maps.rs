use anyhow::Result;
use serde_json::Value;

pub trait Map {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>>;

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>>;
}
