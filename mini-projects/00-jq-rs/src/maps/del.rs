use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct DelMap {
    pub key: String,
}

impl Map for DelMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"del\(\.(\w+)\)";
        let re: &'static Regex = &Regex::new(pattern).unwrap();
    
        match re.captures(input) {
            Some(captures) => {
                let key = captures.get(1).unwrap().as_str();
                Ok(Box::new(DelMap {
                    key: key.to_string(),
                }))
            }
            None => anyhow::bail!("failed to parse del slice"),
        }
    }
}

