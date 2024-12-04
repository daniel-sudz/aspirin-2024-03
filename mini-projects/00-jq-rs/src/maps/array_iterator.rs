use crate::maps::map::Map;
use anyhow::Result;
use serde_json::Value;

// implements the ".[]" operation
// iterates over the array and returns the array elements
// returns an error if the value is not an array
pub struct ArrayIteratorMap {}

impl Map for ArrayIteratorMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        match value.len() {
            0 => Ok(vec![]),
            1 => {
                let value = value[0].clone();
                match value {
                    Value::Array(array) => {
                        let value: Vec<Value> = array.to_vec();
                        Ok(value)
                    }
                    _ => anyhow::bail!("cannot iterate over non-array value"),
                }
            }
            _ => anyhow::bail!("cannot iterate over non-array value"),
        }
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        match input == ".[]" {
            true => Ok(Box::new(ArrayIteratorMap {})),
            false => anyhow::bail!("failed to parse as array iterator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_array_iterator() {
        let array_iterator_map = ArrayIteratorMap {};
        let values: Vec<Value> = vec![serde_json::from_str("[1,2,3]").unwrap()];
        let values = array_iterator_map.map(Ok(values)).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], Value::Number(1.into()));
        assert_eq!(values[1], Value::Number(2.into()));
        assert_eq!(values[2], Value::Number(3.into()));
    }

    #[test]
    fn test_non_array_value() {
        let array_iterator_map = ArrayIteratorMap {};
        let values = array_iterator_map.map(Ok(vec![serde_json::from_str("1").unwrap()]));
        assert_eq!(
            values.unwrap_err().to_string(),
            "cannot iterate over non-array value"
        );
    }
}
