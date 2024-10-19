use anyhow::Result;
use serde_json::Value;

// an abstraction to represent a jq operation
// an operation performs some kind of transformation and is used if it matches the pattern of the command
pub trait Map {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>>;

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>>;
}
