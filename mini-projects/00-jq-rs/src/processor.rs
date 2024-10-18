use serde_json::Value;
use std::fs;
use crate::parser::parse;
use crate::formatter::format;
use crate::args::Args;
use anyhow::Result;

pub fn output_to_string(input: Result<Vec<Value>>, args: &Args) -> Result<String> {
    let input = input?;
    let mut result = String::new();
    for (i, value) in input.iter().enumerate() {
        result += &format(value.clone(),
            args.sort_keys,
          args.indent.unwrap_or(2).into(),
           0,
            args.compact_output, 
            args.monochrome_output)?;
        if i < input.len() - 1 {
            result += "\n";
        }
    }
    Ok(result)
}

pub fn process_from_string(args: &Args) -> Result<String> {
    let json = args.get_input()?;
    let ops = parse(args.command.clone())?;
    let mut result: Result<Vec<Value>> = Ok(vec![json]);
    for op in ops.iter() {
        result = Ok(op.map(result)?);
    }
    output_to_string(result, &args)
}

mod tests{
    use std::path::{absolute, Path};
    use super::*;

    #[test]
    fn test_object_identity_map_example() {
        let args = Args {
            sort_keys: true,
            indent: Some(2),
            compact_output: true,
            monochrome_output: true,
            color_output: false,
            command: ".fizz".to_string(),
            file: Some(absolute(Path::new("sample_data/all_types.json")).unwrap()),
        };
        let res = process_from_string(&args).unwrap();
        assert_eq!(res, "\"buzz\"");
    }

    #[test]
    fn test_array_index_example() {
        let args = Args {
            sort_keys: true,
            indent: Some(2),
            compact_output: true,
            monochrome_output: true,
            color_output: false,
            command: ".[0]".to_string(),
            file: Some(absolute(Path::new("sample_data/array.json")).unwrap()),
        };
        let res = process_from_string(&args).unwrap();
        assert_eq!(res, "\"one\"");
    }

    #[test]
    fn test_array_slice_example() {
        let args = Args {
            sort_keys: true,
            indent: Some(2),
            compact_output: true,
            monochrome_output: true,
            color_output: false,
            command: ".[0:2]".to_string(),
            file: Some(absolute(Path::new("sample_data/array.json")).unwrap()),
        };
        let res = process_from_string(&args).unwrap();
        assert_eq!(res, "[\"one\",\"two\"]");
    }
}