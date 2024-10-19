use crate::maps::maps::Map;
use anyhow::Result;
use regex::Regex;
use serde_json::Value;

pub struct ArrayIndexMap {
    pub index: usize,
}

impl Map for ArrayIndexMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        let result: Result<Vec<Value>> = value
            .iter()
            .map(|v| {
                let array = v
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("array iterator requires an array"))?;
                Ok(array[self.index].clone())
            })
            .collect();
        result
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"\.\[(\d+)\]";
        let re: Regex = Regex::new(pattern).unwrap();

        match re.captures(input) {
            Some(captures) => match captures.get(0).unwrap().as_str() == input {
                true => {
                    let index_str = captures.get(1).unwrap().as_str();
                    match index_str.parse::<usize>() {
                        Ok(index) => {
                            return Ok(Box::new(ArrayIndexMap { index }));
                        }
                        Err(_) => anyhow::bail!("failed to parse array index"),
                    }
                }
                false => anyhow::bail!("failed to parse array index"),
            },
            None => anyhow::bail!("failed to match array index pattern"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // replicates echo "[1,2,3]" | jq ".[1]"
    #[test]
    fn test_basic_array_index() {
        let array_index_map = ArrayIndexMap { index: 1 };
        let values = array_index_map
            .map(Ok(vec![serde_json::from_str("[1,2,3]").unwrap()]))
            .unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "2");
    }

    // replicates echo "[[1,2,3],[4,5,6],[7,8,9]]" | jq ".[] | .[1]"
    #[test]
    fn test_iterator_array_index() {
        let array_index_map = ArrayIndexMap { index: 1 };
        let values: Vec<Value> = vec![
            serde_json::from_str("[1,2,3]").unwrap(),
            serde_json::from_str("[4,5,6]").unwrap(),
            serde_json::from_str("[7,8,9]").unwrap(),
        ];
        let values = array_index_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "2");
        assert_eq!(values[1].to_string(), "5");
        assert_eq!(values[2].to_string(), "8");
    }
}
