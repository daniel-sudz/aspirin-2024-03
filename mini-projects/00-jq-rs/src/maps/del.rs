use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;
use regex::Regex;

pub struct DelMap {
    pub key: String,
}

impl Map for DelMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        value
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"del\(\.(\w+)\)";
        let re: Regex = Regex::new(pattern).unwrap();
    
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

mod tests {
    use super::*;

    // replicates echo '{"a":1,"b":2,"c":3}' | jq "del(.a)"
    #[test]
    fn test_basic_del() {
        let del_map = DelMap { key: "a".to_string() };
        let values = del_map.map(Ok(vec![serde_json::from_str("{\"a\":1,\"b\":2,\"c\":3}").unwrap()])).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "{\"b\":2,\"c\":3}");
    }
}   