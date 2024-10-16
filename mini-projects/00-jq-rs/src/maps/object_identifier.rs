use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct ObjectIdentifierMap {
    pub key: String,
}

impl Map for ObjectIdentifierMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        let result: Result<Vec<Value>> = value.iter().map(|v| {
            let new_value = v[&self.key].clone();
            if new_value.is_null() {
                anyhow::bail!("object identifier not found")
            }
            Ok(new_value)
        }).collect();
        result
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern: &'static str = r"\.\[(\d+):(\d+)\]";
        let re: &'static Regex = &Regex::new(pattern).unwrap();
    
        match re.captures(input) {
            Some(captures) => {
                let first = captures.get(1).unwrap().as_str();
                let second = captures.get(2).unwrap().as_str();
                match (first.parse::<usize>(), second.parse::<usize>()) {
                    (Ok(start), Ok(end)) => {
                        return Ok(Box::new(ArrayIteratorMap {
                            from: start,
                            to: end,
                        }));
                    }
                    _ => anyhow::bail!("failed to parse array slice"),
                }
            },
            None => anyhow::bail!("failed to parse array slice"),
        }
    }
}