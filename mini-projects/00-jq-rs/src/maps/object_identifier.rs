use crate::maps::map::Map;
use anyhow::Result;
use regex::Regex;
use serde_json::Value;
pub struct ObjectIdentifierMap {
    pub key: String,
}

impl Map for ObjectIdentifierMap {
    fn map(&self, value: Result<Vec<Value>>) -> Result<Vec<Value>> {
        let value = value?;
        let result: Result<Vec<Value>> = value
            .iter()
            .map(|v| {
                let new_value = v[&self.key].clone();
                Ok(new_value)
            })
            .collect();
        result
    }

    fn command_match(&self, input: &str) -> Result<Box<dyn Map>> {
        let pattern = r"\.(\w+)";
        let re: Regex = Regex::new(pattern).unwrap();

        match re.captures(input) {
            Some(captures) => match captures.get(0).unwrap().as_str() == input {
                true => {
                    let key = captures.get(1).unwrap().as_str();
                    Ok(Box::new(ObjectIdentifierMap {
                        key: key.to_string(),
                    }))
                }
                false => anyhow::bail!("failed to parse object identifier"),
            },
            None => anyhow::bail!("failed to parse object identifier"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_object_identifier() {
        let object_identifier_map = ObjectIdentifierMap {
            key: "foo".to_string(),
        };
        let values = object_identifier_map
            .map(Ok(vec![serde_json::from_str(r#"{"foo": "bar"}"#).unwrap()]))
            .unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), r#""bar""#);
    }

    #[test]
    fn test_non_existent_object_identifier() {
        let object_identifier_map = ObjectIdentifierMap {
            key: "foobar".to_string(),
        };
        let values = object_identifier_map
            .map(Ok(vec![serde_json::from_str(r#"{"foo": "bar"}"#).unwrap()]))
            .unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].to_string(), r#"null"#);
    }
}
