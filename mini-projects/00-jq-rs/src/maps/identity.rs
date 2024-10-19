use crate::maps::maps::Map;
use anyhow::Result;
use serde_json::Value;
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

mod tests {
    use super::*;

    // replicates echo '[0,1,2]' | jq "."
    #[test]
    fn test_basic_identity() {
        let identity_map = IdentityMap;
        let values = identity_map
            .map(Ok(vec![serde_json::from_str("[0,1,2]").unwrap()]))
            .unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "[0,1,2]");
    }
}
