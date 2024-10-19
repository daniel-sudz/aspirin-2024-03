use crate::maps::maps::Map;
use anyhow::{Context, Result};
use serde_json::Value;

pub struct AddMap {}

impl Map for AddMap {
    fn map(&self, values: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let values = values?;
        if values.len() != 1 {
            anyhow::bail!("expected array but recieved iterator");
        }
        let value = values[0].as_array().context("expected array")?;

        let mut sum: Value = value[0].clone();

        for value in value[1..].iter() {
            sum = match (&sum, &value) {
                // null consumes anything to null
                (Value::Null, _) => sum.clone(),
                (_, Value::Null) => Value::from(()),

                // number adds with numbers
                (Value::Number(lhs), Value::Number(rhs)) => {
                    match (lhs.is_i64() || lhs.is_u64(), rhs.is_i64() || rhs.is_u64()) {
                        (true, true) => Value::from(lhs.as_i64().unwrap() + rhs.as_i64().unwrap()),
                        (_, _) => Value::from(lhs.as_f64().unwrap() + rhs.as_f64().unwrap()),
                    }
                }
                (Value::Number(lhs), rhs) => {
                    Value::from(lhs.to_string() + rhs.to_string().as_str())
                }

                // booleans add with booleans via xor
                (Value::Bool(lhs), Value::Bool(rhs)) => Value::from(lhs ^ rhs),

                // string upcasts everything to string
                (Value::String(lhs), Value::String(rhs)) => Value::from(lhs.clone() + rhs.as_str()),
                (Value::String(lhs), rhs) => Value::from(lhs.clone() + rhs.to_string().as_str()),
                (lhs, Value::String(rhs)) => {
                    Value::from(lhs.to_string() + rhs.to_string().as_str())
                }

                // arrays concat with arrays and otherwise give error
                (Value::Array(lhs), Value::Array(rhs)) => Value::from(
                    lhs.iter()
                        .cloned()
                        .chain(rhs.iter().cloned())
                        .collect::<Vec<Value>>(),
                ),
                (Value::Array(_), _) | (_, Value::Array(_)) => {
                    anyhow::bail!("cannot add array with non-array")
                }

                // objects fail to concat
                (Value::Object(_), _) => {
                    anyhow::bail!("cannot add object with non-object");
                }

                // otherwise fail
                _ => anyhow::bail!("unexpected value type for addition: {}", value),
            }
        }
        Ok(vec![sum])
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        match input == "add" {
            true => Ok(Box::new(AddMap {})),
            false => anyhow::bail!("failed to parse add"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_fails() {
        let add_map = AddMap {};
        let values: Vec<Value> = vec![Value::from("1"), Value::from("2")];
        let value = add_map.map(Ok(values));
        assert_eq!(
            value.unwrap_err().to_string(),
            "expected array but recieved iterator"
        );
    }

    #[test]
    fn test_add_integers() {
        let add_map = AddMap {};
        let values: Vec<Value> = vec![Value::from(vec![1, 2])];
        let value = add_map.map(Ok(values));
        assert_eq!(value.unwrap()[0], 3);
    }

    #[test]
    fn test_mixed_integer_floats() {
        let add_map = AddMap {};
        let values: Vec<Value> = vec![serde_json::from_str("[1,2.0,3]").unwrap()];
        let value = add_map.map(Ok(values));
        assert_eq!(value.unwrap()[0], 6.0);
    }

    #[test]
    fn test_string_concat() {
        let add_map = AddMap {};
        let values: Vec<Value> = vec![serde_json::from_str("[\"a\",\"b\"]").unwrap()];
        let value = add_map.map(Ok(values));
        assert_eq!(value.unwrap()[0], "ab");
    }

    #[test]
    fn test_array_concat() {
        let add_map = AddMap {};
        let values: Vec<Value> = vec![serde_json::from_str("[[1,2],[3,4]]").unwrap()];
        let value = add_map.map(Ok(values));
        assert_eq!(value.unwrap()[0].to_string(), "[1,2,3,4]");
    }
}
