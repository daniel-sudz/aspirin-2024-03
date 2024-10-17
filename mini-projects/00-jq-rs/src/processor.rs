use serde_json::Value;
use std::fs;
use crate::parser::parse;
use anyhow::Result;

pub fn output_to_string(input: Result<Vec<Value>>) -> Result<String> {
    let input = input?;
    let mut result = String::new();
    for value in input.iter() {
        result += &serde_json::to_string(value)?;
    }
    Ok(result)
}

pub fn process_from_string(process_string: String, input: String) -> Result<String> {
    let json: Value = serde_json::from_str(&input)?;
    let ops = parse(process_string)?;
    let mut result: Result<Vec<Value>> = Ok(vec![json]);

    for op in ops.iter() {
        result = Ok(op.map(result)?);
    }
    output_to_string(result)
}

mod tests{
    use super::*;

    const ALL_TYPES: &str = r#"
    {
    "fizz": "buzz",
    "baz": null,
    "fuzz": true,
    "bizz": 22.0,
    "biz": 42,
    "fizzes": [
        "buzz",
        null,
        true,
        22.0,
        42.0
    ]
}
    "#;

    #[test]
    fn test_object_identity_map_example() {
        let input = ".fizz".to_string();
        let res = process_from_string(input, ALL_TYPES.to_string()).unwrap();
        assert_eq!(res, "\"buzz\"");
    }

    #[test]
    fn test_array_index_example() {
        let input = ".[0]".to_string();
        let res = process_from_string(input, ALL_TYPES.to_string()).unwrap();
        assert_eq!(res, "\"one\"");
    }
}