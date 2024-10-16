use serde_json::Value;
use anyhow::Result;


pub trait Map {
    fn map(&self, value: Result<Value>) -> Result<Value>;
}


