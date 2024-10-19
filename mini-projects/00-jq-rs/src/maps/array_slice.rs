use crate::maps::map::Map;
use anyhow::Result;
use regex::Regex;
use serde_json::Value;
use std::cmp::min;

pub struct ArraySliceMap {
    pub from: usize,
    pub to: usize,
}

impl Map for ArraySliceMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;

        // when testing against the official jq the array slice can work with both iterators and array indexing
        // for iterators it slices every element in the iterator
        let result: Result<Vec<Value>> = value
            .iter()
            .map(|v| match v {
                Value::Array(array) => Ok(Value::Array(
                    array[self.from..min(self.to, array.len())].to_vec(),
                )),
                Value::String(string) => Ok(Value::String(
                    string[self.from..min(self.to, string.len())].to_string(),
                )),
                _ => anyhow::bail!("cannot index non-array value"),
            })
            .collect();
        result
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"\.\[(\d+):(\d+)\]";
        let re: Regex = Regex::new(pattern).unwrap();

        match re.captures(input) {
            Some(captures) => match captures.get(0).unwrap().as_str() == input {
                true => {
                    let first = captures.get(1).unwrap().as_str();
                    let second = captures.get(2).unwrap().as_str();
                    match (first.parse::<usize>(), second.parse::<usize>()) {
                        (Ok(start), Ok(end)) => Ok(Box::new(ArraySliceMap {
                            from: start,
                            to: end,
                        })),
                        _ => anyhow::bail!("failed to parse array slice"),
                    }
                }
                false => anyhow::bail!("failed to parse array slice"),
            },
            None => anyhow::bail!("failed to parse array slice"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // replicates echo "[1,2,3]" | jq ".[0:2]"
    #[test]
    fn test_basic_array_slice() {
        let array_slice_map = ArraySliceMap { from: 0, to: 2 };
        let values: Vec<Value> = vec![serde_json::from_str("[1,2,3]").unwrap()];
        let values = array_slice_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "[1,2]");
    }

    // replicates echo '"foo"' | jq ".[0:2]"
    #[test]
    fn test_basic_string_slice() {
        let array_slice_map = ArraySliceMap { from: 0, to: 2 };
        let values: Vec<Value> = vec![Value::String("foo".to_string())];
        let values = array_slice_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "\"fo\"");
    }

    // replicates echo '["one", "two", "three"]' | jq ".[] | .[0:2]"
    #[test]
    fn test_iterator_array_slice_1() {
        let array_slice_map = ArraySliceMap { from: 0, to: 2 };
        let values: Vec<Value> = vec![
            Value::String("one".to_string()),
            Value::String("two".to_string()),
            Value::String("three".to_string()),
        ];
        let values = array_slice_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "\"on\"");
        assert_eq!(values[1].to_string(), "\"tw\"");
        assert_eq!(values[2].to_string(), "\"th\"");
    }

    // replicates echo '[[1,2,3], [4,5,6], [7,8,9]]' | jq ".[] | .[0:2]"
    #[test]
    fn test_iterator_array_slice_2() {
        let array_slice_map = ArraySliceMap { from: 0, to: 2 };
        let values: Vec<Value> = vec![
            serde_json::from_str("[1,2,3]").unwrap(),
            serde_json::from_str("[4,5,6]").unwrap(),
            serde_json::from_str("[7,8,9]").unwrap(),
        ];
        let values = array_slice_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "[1,2]");
        assert_eq!(values[1].to_string(), "[4,5]");
        assert_eq!(values[2].to_string(), "[7,8]");
    }

    // replicates echo '1' | jq ".[0:2]"
    #[test]
    fn test_basic_non_iterable_slice() {
        let array_slice_map = ArraySliceMap { from: 0, to: 2 };
        let values: Vec<Value> = vec![Value::Number(1.into())];
        let values = array_slice_map.map(Ok(values));
        assert_eq!(
            values.err().unwrap().to_string(),
            "cannot index non-array value"
        );
    }

    // replicates echo '[1,2,3]' | jq '.[0:100]'
    #[test]
    fn test_out_of_bounds_upper() {
        let array_slice_map = ArraySliceMap { from: 0, to: 100 };
        let values: Vec<Value> = vec![serde_json::from_str("[1,2,3]").unwrap()];
        let values = array_slice_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "[1,2,3]");
    }
}
