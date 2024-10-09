use crate::args::Args;
use anyhow::Result;
use std::io::BufRead;

// a reader than takes in the input and outputs an iterator of lines
pub trait Reader {
    fn read(&mut self, args: &Args) -> Box<dyn Iterator<Item = Result<String>>>;
}

// case sensitive preprocessor
pub struct BuffReader;

// buffered reader from disk
impl Reader for BuffReader {
    fn read(&mut self, args: &Args) -> Box<dyn Iterator<Item = Result<String>>> {
        let file = &args.file;
        match file {
            // handle reading from file
            Some(f) => {
                let file = std::fs::File::open(f).unwrap();
                let reader = std::io::BufReader::new(file);
                let lines = reader.lines().map(|l| l.map_err(|e| e.into()));
                Box::new(lines)
            }
            // handle reading from stdin
            None => Box::new(std::io::stdin().lines().map(|l| l.map_err(|e| e.into()))),
        }
    }
}

// provides a reader from a vector of strings
pub struct MemoryReader {
    pub input: Vec<String>,
}

impl Reader for MemoryReader {
    fn read(&mut self, _: &Args) -> Box<dyn Iterator<Item = Result<String>>> {
        let mut input_lines: Vec<String> = vec![];
        std::mem::swap(&mut input_lines, &mut self.input);
        Box::new(
            input_lines
                .into_iter()
                .map(Ok::<String, anyhow::Error>),
        )
    }
}

// a writer than outputs to the standard output/error
pub trait Writer<'a> {
    fn write(&mut self, stream: Box<dyn Iterator<Item = Result<String>> + 'a>);
}

// writes to to the stdout, putting errors into stderr
pub struct StdoutWriter;

impl<'a> Writer<'a> for StdoutWriter {
    fn write(&mut self, stream: Box<dyn Iterator<Item = Result<String>> + 'a>) {
        for line in stream {
            match line {
                Ok(l) => println!("{}", l),
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}

// writes to a vector of strings
pub struct MemoryWriter<'a> {
    pub output: &'a mut Vec<String>,
    pub error: &'a mut Vec<String>,
}

impl<'a> Writer<'a> for MemoryWriter<'a> {
    fn write(&mut self, stream: Box<dyn Iterator<Item = Result<String>> + 'a>) {
        for line in stream {
            match line {
                Ok(l) => self.output.push(l),
                Err(e) => self.error.push(e.to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{absolute, Path};
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_reader() {
        let mut tf = NamedTempFile::new().unwrap();
        tf.write("one\ntwo\nthree".as_bytes());
        tf.flush();

        let args = Args {
            ignore_case: false,
            invert_match: false,
            regex: false,
            color: None,
            needle: "".to_string(),
            file: Some(tf.path().to_path_buf()),
        };

        let mut reader: Box<dyn Reader> = Box::new(BuffReader);

        let from_disk: Vec<String> = reader
            .read(&args)
            .map(|x| match x {
                Ok(x) => x,
                Err(e) => e.to_string(),
            })
            .collect();

        assert_eq!(from_disk, vec!["one", "two", "three"]);
    }
}
