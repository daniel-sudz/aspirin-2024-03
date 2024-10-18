use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;
use regex::Regex;

pub struct ArrayIndexMap {
    pub index: usize,
}

impl Map for ArrayIndexMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        let result: Result<Vec<Value>> = value.iter().map(|v| {
            let array = v.as_array().ok_or_else(|| anyhow::anyhow!("array iterator requires an array"))?;
            Ok(array[self.index].clone())
        }).collect();
        result
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"\.\[(\d+)\]";
        let re: Regex = Regex::new(pattern).unwrap();
    
        match re.captures(input) {
            Some(captures) => {
                let index_str = captures.get(1).unwrap().as_str();
                match index_str.parse::<usize>() {
                    Ok(index) => {
                        return Ok(Box::new(ArrayIndexMap { index }));
                    }
                    Err(_) => anyhow::bail!("failed to parse array index"),
                }
            },
            None => anyhow::bail!("failed to match array index pattern"),
        }
    }
}

