use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;
use regex::Regex;

pub struct DelMapArray {
    pub from: usize,
    pub to: usize,
}

impl Map for DelMapArray {
    fn map(&self, values: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let values = values?;
        values.iter().map(|v| {
            match v {
                Value::Array(arr) => {
                    let mut arr = arr.clone();
                    arr.drain(self.from..self.to);
                    Ok(Value::Array(arr))
                }
                _ => anyhow::bail!("cannot delete non-array value"),
            }
        }).collect()
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"del\(\.\[(\d+):(\d+)\]\)";
        let re: Regex = Regex::new(pattern).unwrap();
    
        match re.captures(input) {
            Some(captures) => {
                match captures.get(0).unwrap().as_str() == input {
                    true => {
                        let from = captures.get(1).unwrap().as_str();
                        let to = captures.get(2).unwrap().as_str();
                        Ok(Box::new(DelMapArray {
                            from: from.parse().unwrap(),
                        to: to.parse().unwrap(),
                    }))
                    }
                    false => anyhow::bail!("failed to parse del slice"),
                }
            },
            None => anyhow::bail!("failed to parse del slice"),
        }
    }
}

mod tests {
    use super::*;

    // replicates echo '[0,1,2]' | jq "del(.[0,1])"
    #[test]
    fn test_basic_del_array() {
        let del_map = DelMapArray { from: 0, to: 1 };
        let values = del_map.map(Ok(vec![serde_json::from_str("[0,1,2]").unwrap()])).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "[1,2]");
    }

}   