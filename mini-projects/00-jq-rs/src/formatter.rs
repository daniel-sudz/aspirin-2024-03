use anyhow::Result;
use serde_json::{json, Value};
use std::env;

enum JqColorType {
    Null,
    False,
    True,
    Number,
    String,
    Array,
    Object,
    ObjectKey,
}

fn get_jq_colors() -> Result<Vec<String>> {
    let jq_colors =
        env::var("JQ_COLORS").unwrap_or("0;90:0;37:0;37:0;37:0;32:1;37:1;37:1;34".to_string());
    let jq_colors: Vec<String> = jq_colors.split(":").map(|s| s.to_string()).collect();
    if jq_colors.len() != 8 {
        return Err(anyhow::anyhow!("Invalid JQ_COLORS supplied"));
    }
    Ok(jq_colors)
}

fn color_string(input: &str, t: JqColorType, disable: bool) -> Result<String> {
    if disable {
        return Ok(input.to_string());
    }
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

pub fn format(
    input: Value,
    sort_keys: bool,
    indent: usize,
    cur_indent: usize,
    compact: bool,
    disable_colors: bool,
) -> Result<String> {
    match input {
        Value::Null => color_string("null", JqColorType::Null, disable_colors),
        Value::Bool(b) => match b {
            true => color_string("true", JqColorType::True, disable_colors),
            false => color_string("false", JqColorType::False, disable_colors),
        },
        Value::Number(n) => color_string(&n.to_string(), JqColorType::Number, disable_colors),
        Value::String(s) => {
            color_string(&format!("\"{}\"", s), JqColorType::String, disable_colors)
        }
        Value::Array(a) => {
            let mut result: String = color_string("[", JqColorType::Array, disable_colors)?;
            if !compact {
                result.push_str("\n");
                result.push_str(&" ".repeat(cur_indent + indent));
            }
            for (i, e) in a.iter().enumerate() {
                let formatted_element = format(
                    e.clone(),
                    sort_keys,
                    indent,
                    cur_indent + indent,
                    compact,
                    disable_colors,
                )?;
                result.push_str(&formatted_element);
                if i < a.len() - 1 {
                    result.push_str(",");
                    if !compact {
                        result.push_str("\n");
                        result.push_str(&" ".repeat(cur_indent + indent));
                    }
                }
            }
            if !compact {
                result.push_str("\n");
                result.push_str(&" ".repeat(cur_indent));
            }
            result.push_str(&color_string("]", JqColorType::Array, disable_colors)?);
            Ok(result)
        }
        Value::Object(o) => {
            let mut result: String = color_string("{", JqColorType::Object, disable_colors)?;
            if !compact {
                result.push_str("\n");
                result.push_str(&" ".repeat(cur_indent + indent));
            }
            let mut keys: Vec<String> = o.keys().into_iter().map(|k| k.to_string()).collect();
            match sort_keys {
                true => keys.sort(),
                false => (),
            }
            for (i, k) in keys.iter().enumerate() {
                let v = o.get(k).unwrap().clone();
                result.push_str(&color_string(
                    &format!("\"{k}\""),
                    JqColorType::ObjectKey,
                    disable_colors,
                )?);
                result.push_str(":");
                if !compact {
                    result.push_str(" ");
                }
                result.push_str(&format(
                    v,
                    sort_keys,
                    indent,
                    cur_indent + indent,
                    compact,
                    disable_colors,
                )?);
                if i < &keys.len() - 1 {
                    if compact {
                        result.push_str(",");
                    } else {
                        result.push_str(",\n");
                        result.push_str(&" ".repeat(cur_indent + indent));
                    }
                }
            }
            if !compact {
                result.push_str("\n");
                result.push_str(&" ".repeat(cur_indent));
            }
            result.push_str(&color_string("}", JqColorType::Object, disable_colors)?);
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::samples::{ALL_TYPES, ARRAY};

    // MATCHES JQ_COLORS="::::::::" jq "." all_types.json
    #[test]
    fn test_basic_format() {
        env::set_var("JQ_COLORS", ":::::::");
        let input: Value = serde_json::from_str(ALL_TYPES).unwrap();
        let formatted = format(input, false, 2, 0, false, true).unwrap();
        let expected = r#"{
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
}"#;
        assert_eq!(formatted, expected);
    }

    // MATCHES JQ_COLORS="::::::::" jq --sort-keys "." all_types.json
    #[test]
    fn test_sort_keys() {
        env::set_var("JQ_COLORS", ":::::::");
        let input: Value = serde_json::from_str(ALL_TYPES).unwrap();
        let formatted = format(input, true, 2, 0, false, true).unwrap();
        let expected = r#"{
  "baz": null,
  "biz": 42,
  "bizz": 22.0,
  "fizz": "buzz",
  "fizzes": [
    "buzz",
    null,
    true,
    22.0,
    42.0
  ],
  "fuzz": true
}"#;
        assert_eq!(formatted, expected);
    }

    // MATCHES JQ_COLORS="::::::::" jq --indent 7 --sort-keys "." all_types.json
    #[test]
    fn test_custom_indent() {
        env::set_var("JQ_COLORS", ":::::::");
        let input: Value = serde_json::from_str(ALL_TYPES).unwrap();
        let formatted = format(input, true, 7, 0, false, true).unwrap();
        let expected = r#"{
       "baz": null,
       "biz": 42,
       "bizz": 22.0,
       "fizz": "buzz",
       "fizzes": [
              "buzz",
              null,
              true,
              22.0,
              42.0
       ],
       "fuzz": true
}"#;
        assert_eq!(formatted, expected);
    }

    // MATCHES JQ_COLORS="::::::::" jq --sort-keys --compact-output "." all_types.json
    #[test]
    fn test_compact_output() {
        env::set_var("JQ_COLORS", ":::::::");
        let input: Value = serde_json::from_str(ALL_TYPES).unwrap();
        let formatted = format(input, true, 2, 0, true, true).unwrap();
        let expected = r#"{"baz":null,"biz":42,"bizz":22.0,"fizz":"buzz","fizzes":["buzz",null,true,22.0,42.0],"fuzz":true}"#;
        assert_eq!(formatted, expected);
    }
}
