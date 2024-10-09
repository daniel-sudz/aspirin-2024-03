use clap::Parser;

pub mod args;
pub mod io;
pub mod processor;
pub mod transformers;

use args::Args;

fn main() {
    let args = Args::parse();
    processor::disk_processor_factory(args);
}

#[cfg(test)]
mod tests {
    use std::path::{absolute, Path};
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    use tempfile::NamedTempFile;

    #[test]
    fn test_stdin_mode() {
        let exe_path = absolute(
            Path::new(file!())
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("target/debug/greprs"),
        )
        .unwrap();
        let exe_string = exe_path.to_str().unwrap();
        let mut proc = Command::new(exe_string)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("test")
            .spawn()
            .unwrap();

        {
            let mut proc_in = proc.stdin.take().unwrap();
            let _ = proc_in.write("test\n no match \ntest abc\n no match \n test123".as_bytes());
        }

        let res = proc.wait_with_output().unwrap();
        let stdout = String::from_utf8(res.stdout).unwrap();
        let stderr = String::from_utf8(res.stderr).unwrap();

        assert_eq!(stdout, "test\ntest abc\n test123\n");
        assert_eq!(stderr, "");
    }

    #[test]
    fn test_file_mode() {
        let mut tf = NamedTempFile::new().unwrap();
        let _ = tf.write("test\n no match \ntest abc\n no match \n test123".as_bytes());
        let _ = tf.flush();

        let exe_path = absolute(
            Path::new(file!())
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("target/debug/greprs"),
        )
        .unwrap();
        let exe_string = exe_path.to_str().unwrap();
        let proc = Command::new(exe_string)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("test")
            .arg(absolute(tf.path()).unwrap().to_str().unwrap())
            .spawn()
            .unwrap();

        let res = proc.wait_with_output().unwrap();
        let stdout = String::from_utf8(res.stdout).unwrap();
        let stderr = String::from_utf8(res.stderr).unwrap();

        assert_eq!(stderr, "");
        assert_eq!(stdout, "test\ntest abc\n test123\n");
    }
}
