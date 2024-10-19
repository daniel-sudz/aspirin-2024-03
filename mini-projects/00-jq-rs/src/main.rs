mod args;
mod parser;
mod processor;
mod formatter;
mod samples;
use args::Args;
use clap::Parser;
use serde_json::Value;
use anyhow::Result;
use std::fs;
pub mod maps;

fn main() -> Result<()> {
    let args = Args::parse();
    let output = processor::process_from_string(&args)?;
    println!("{output}");
    Ok(())
}

mod tests {
    use std::{path::{absolute, Path, PathBuf}, process::{Command, Stdio}};
    use anyhow::Result;
    use super::*;

    // rebuild the debug binary before using it
    fn rebuild_binary() -> Result<()> {
        let root_dir_absolute = absolute(".")?;
        let root_dir_string = root_dir_absolute.to_str().unwrap();

        let mut proc = Command::new("cargo")
        .current_dir(root_dir_string)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("build")
        .spawn()?; 

        let res = proc.wait()?;
        if !res.success() {
            panic!("Failed to rebuild binary");
        }
        println!("Rebuilt binary");
        Ok(())
    }
    
    // test the cli binary directly with full integration   
    fn test_from_sample_data(file_name: &str, args: Vec<&str>, expected_stdout: &str, expected_stderr: &str) -> Result<()> {
        rebuild_binary()?;
        let exe_path = absolute(
            Path::new("target/debug/jq-rs")
        )?;
        let exe_string = exe_path.to_str().unwrap();

        let input_file = absolute(Path::new("sample_data").join(file_name))?;
        let input_string = input_file.to_str().unwrap();

        let mut args = args;
        args.push(input_string);
        let proc = Command::new(exe_string)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
            .spawn()?;

        let res = proc.wait_with_output()?;
        let stdout = String::from_utf8(res.stdout).unwrap();
        let stderr = String::from_utf8(res.stderr).unwrap();
        assert_eq!(stdout, expected_stdout);
        assert_eq!(stderr, expected_stderr);
        Ok(())
    }

    #[test]
    fn test_all_type_identity() {
        let args: Vec<&str> = vec!["--monochrome-output", "."];
        let expected_stdout =
r#"{
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
        test_from_sample_data("all_types.json", args, expected_stdout, "").unwrap();
    }

    #[test]
    fn test_add_pipe_example() {
        let args: Vec<&str> = vec!["--monochrome-output", "--compact-output", ". | add"];
        let expected_stdout = "\"onetwothree\"\n";
        test_from_sample_data("array.json", args, expected_stdout, "").unwrap();
    }


    // replicates jq ". | del(.fizzes)" all_types.json --compact-output --sort-keys --monochrome-output
    #[test]
    fn test_del_fizzes() {
        let args: Vec<&str> = vec!["--monochrome-output", "--compact-output", "--sort-keys", ". | del(.fizzes)"];
        let expected_stdout = r#"{"baz":null,"biz":42,"bizz":22.0,"fizz":"buzz","fuzz":true}
"#;
        test_from_sample_data("all_types.json", args, expected_stdout, "").unwrap();
    }

    // replicates jq "del(.[0:1])" array.json --monochrome-output --compact-output --sort-keys
    #[test]
    fn test_del_array_slice() {
        let args: Vec<&str> = vec!["--monochrome-output", "--compact-output", "sort-keys", ". | del(.[0:1])"];
        let expected_stdout = r#"["two", "three"]"#;
        test_from_sample_data("array.json", args, expected_stdout, "").unwrap();
    }
}