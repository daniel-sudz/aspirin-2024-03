use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct ArrayIteratorMap {
    pub from: usize,
    pub to: usize,
}

impl Map for ArrayIteratorMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        let result: Result<Vec<Value>> = value.iter().map(|v| {
            let array = v.as_array().ok_or_else(|| anyhow::anyhow!("array iterator requires an array"))?;
            let slice = array[self.from..self.to].to_vec();
            Ok(Value::Array(slice))
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

