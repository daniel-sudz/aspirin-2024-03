use serde_json::{Value, json};
use std::env;
use anyhow::Result;

enum JqColorType {
    Null,
    False,
    True,
    Number,
    String,
    Array,
    Object,
    ObjectKey
}

fn get_jq_colors() -> Result<Vec<String>> {
    let jq_colors = env::var("JQ_COLORS").unwrap_or("0;90:0;37:0;37:0;37:0;32:1;37:1;37:1;34".to_string());
    let jq_colors: Vec<String> = jq_colors.split(":").map(|s| s.to_string()).collect();
    if jq_colors.len() != 8 {
        return Err(anyhow::anyhow!("Invalid JQ_COLORS supplied"));
    }
    Ok(jq_colors)
}

fn color_string(input: &str, t: JqColorType) -> Result<String> {
        let jq_colors = get_jq_colors()?;
        let color_code: &String = match t {
        JqColorType::Null => &jq_colors[0],
        JqColorType::False => &jq_colors[1],
        JqColorType::True => &jq_colors[2],
        JqColorType::Number => &jq_colors[3],
        JqColorType::String => &jq_colors[4],
        JqColorType::Array => &jq_colors[5],
        JqColorType::Object => &jq_colors[6],
        JqColorType::ObjectKey => &jq_colors[7],
    };
    Ok(format!("\x1b[{color_code}m{input}\x1b[0m"))
}

pub fn format(input: Value, sort_keys: bool) -> Result<String> {
    match input {
        Value::Null => color_string("null", JqColorType::Null),
        Value::Bool(b) => {
            match b {
                true => color_string("true", JqColorType::True),
                false => color_string("false", JqColorType::False),
            }
        },
        Value::Number(n) => color_string(&n.to_string(), JqColorType::Number),
        Value::String(s) => color_string(&format!("\"{}\"", s), JqColorType::String),
        Value::Array(a) => {
            let mut result: String = color_string("[", JqColorType::Array)?;
            for (i, e) in a.iter().enumerate() {
                let formatted_element = format(e.clone(), sort_keys)?;
                result.push_str(&formatted_element);
                if i < a.len() - 1 {
                    result.push_str(", ");
                }
            }
            result.push_str(&color_string("]", JqColorType::Array)?);
            Ok(result)
        }
        Value::Object(o) => {
            let mut result: String = color_string("{", JqColorType::Object)?;
            let mut keys: Vec<String> = o.keys().into_iter().map(|k| k.to_string()).collect();
            match sort_keys {
                true => keys.sort(),
                false => ()
            }
            for (i, k) in keys.iter().enumerate() {
                let v = o.get(k).unwrap().clone();
                result.push_str(&color_string(&format!("{k}"), JqColorType::ObjectKey)?);
                result.push_str(": ");
                result.push_str(&format(v, sort_keys)?);
                if i < &keys.len() - 1 {
                    result.push_str(",\n");
                }
            }
            result.push_str(&color_string("}", JqColorType::Object)?);
            Ok(result)
        }
    }
}

mod tests {
    use super::*;
    use crate::samples::{ALL_TYPES, ARRAY};

    #[test]
    fn test_basic_format() {
        let input: Value = serde_json::from_str(ALL_TYPES).unwrap();
        let formatted = format(input, true).unwrap();
        println!("{}", formatted);
    } 
}