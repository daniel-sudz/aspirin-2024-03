#[cfg(test)]
mod tests {
    use anyhow::Result;
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;
    use std::{
        env,
        path::{absolute, Path},
        process::Stdio,
    };

    // test the cli binary directly with full integration
    fn test_from_sample_data(
        file_name: &str,
        args: Vec<&str>,
        expected_stdout: &str,
        expected_stderr: &str,
    ) -> Result<()> {
        let input_file = absolute(Path::new("sample_data").join(file_name))?;
        let input_string = input_file.to_str().unwrap();

        let mut cmd = Command::cargo_bin("jq-rs")?;
        cmd.args(args).arg(input_string);

        cmd.assert()
            .success()
            .stdout(predicate::str::contains(expected_stdout))
            .stderr(predicate::str::contains(expected_stderr));

        Ok(())
    }

    #[test]
    fn test_all_type_identity() {
        let args: Vec<&str> = vec!["--monochrome-output", "."];
        let expected_stdout = r#"{
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
        let args: Vec<&str> = vec![
            "--monochrome-output",
            "--compact-output",
            "--sort-keys",
            ". | del(.fizzes)",
        ];
        let expected_stdout = r#"{"baz":null,"biz":42,"bizz":22.0,"fizz":"buzz","fuzz":true}
"#;
        test_from_sample_data("all_types.json", args, expected_stdout, "").unwrap();
    }

    // replicates jq "del(.[0:1])" array.json --monochrome-output --compact-output --sort-keys
    #[test]
    fn test_del_array_slice() {
        let args: Vec<&str> = vec![
            "--monochrome-output",
            "--compact-output",
            "--sort-keys",
            ". | del(.[0:1])",
        ];
        let expected_stdout = r#"["two","three"]
"#;
        test_from_sample_data("array.json", args, expected_stdout, "").unwrap();
    }
}
