use crate::maps::map::Map;
use anyhow::Result;
use regex::Regex;
use serde_json::Value;

pub struct DelMapKey {
    pub key: String,
}

impl Map for DelMapKey {
    fn map(&self, values: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let values = values?;
        values
            .iter()
            .map(|v| match v {
                Value::Object(obj) => {
                    let mut obj = obj.clone();
                    obj.remove(&self.key);
                    Ok(Value::Object(obj))
                }
                _ => anyhow::bail!("cannot delete non-object value"),
            })
            .collect()
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"del\(\.(\w+)\)";
        let re: Regex = Regex::new(pattern).unwrap();

        match re.captures(input) {
            Some(captures) => match captures.get(0).unwrap().as_str() == input {
                true => {
                    let key = captures.get(1).unwrap().as_str();
                    Ok(Box::new(DelMapKey {
                        key: key.to_string(),
                    }))
                }
                false => anyhow::bail!("failed to parse del key"),
            },
            None => anyhow::bail!("failed to parse del key"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // replicates echo '{"a":1,"b":2}' | jq "del(.a)"
    #[test]
    fn test_basic_del() {
        let del_map = DelMapKey {
            key: "a".to_string(),
        };
        let values = del_map
            .map(Ok(vec![serde_json::from_str("{\"a\":1,\"b\":2}").unwrap()]))
            .unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "{\"b\":2}");
    }

    // replicates echo '{"a":1}' | jq "del(.f)"
    #[test]
    fn test_del_nonexistent_key() {
        let del_map = DelMapKey {
            key: "f".to_string(),
        };
        let values = del_map
            .map(Ok(vec![serde_json::from_str("{\"a\":1}").unwrap()]))
            .unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "{\"a\":1}");
    }

    // replicates echo '{"a":1, "b": {"c": 2}}' | jq "del(.b)"
    #[test]
    fn test_del_nested_object() {
        let del_map = DelMapKey {
            key: "b".to_string(),
        };
        let values = del_map
            .map(Ok(vec![serde_json::from_str(
                "{\"a\":1, \"b\": {\"c\": 2}}",
            )
            .unwrap()]))
            .unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), "{\"a\":1}");
    }
}
