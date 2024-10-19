use serde_json::Value;
use crate::maps::maps::Map;
use anyhow::Result;

pub struct LengthMap {
}

impl Map for LengthMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let values = value?;
        let values: Result<Vec<Value>> = values.iter().map(|v| {
            match v {
                Value::Array(arr) => Ok(Value::Number(arr.len().into())),
                Value::Object(obj) => Ok(Value::Number(obj.len().into())),
                Value::String(s) => Ok(Value::Number(s.len().into())),
                _ => anyhow::bail!("cannot get length of value"),
            }
        }).collect();
        values
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        match input == "length" {
            true => Ok(Box::new(LengthMap {})),
            false => anyhow::bail!("failed to parse length"),
        }
    }
}

mod tests {
    use super::*;

    // replicates echo '[0,1,2]' | jq "length"
    #[test]
    fn test_basic_length() {
        let length_map = LengthMap {};
        let values = length_map.map(Ok(vec![serde_json::from_str("[0,1,2]").unwrap()])).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0], 3);
    }

    // replicates echo '[[0],[1],[2]]' | jq ".[] | length"
    #[test]
    fn test_length_array() {
        let length_map = LengthMap {};
        let values = vec![
            serde_json::from_str("[0,1]").unwrap(),
            serde_json::from_str("[2,3]").unwrap(),
            serde_json::from_str("[5,6]").unwrap(),
        ];
        let values = length_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], 2);
        assert_eq!(values[1], 2);
        assert_eq!(values[2], 2);
    }


    // replicates echo '"foobar"' | jq "length" 
    #[test]
    fn test_length_string() {
        let length_map = LengthMap {};
        let values = length_map.map(Ok(vec![serde_json::from_str("\"foobar\"").unwrap()])).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0], 6);
    }

    // replicates echo '{"a": 1, "b": 2, "c": 3}' | jq "length"
    #[test]
    fn test_length_object() {
        let length_map = LengthMap {};
        let values = length_map.map(Ok(vec![serde_json::from_str("{\"a\": 1, \"b\": 2, \"c\": 3}").unwrap()])).unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0], 3);
    }
}