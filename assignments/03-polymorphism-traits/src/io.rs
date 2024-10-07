use std::{collections::VecDeque, io::BufRead};
use anyhow::Result;
use crate::args::Args;
use std::mem::swap;

// a reader than takes in the input and outputs an iterator of lines
pub trait Reader {
    fn read(&mut self, args: &Args) -> Box<dyn Iterator<Item = Result<String>>>;
}

// case sensitive preprocessor 
pub struct BuffReader;

// buffered reader from disk
impl Reader for BuffReader {
    fn read(&mut self, args: &Args) -> Box<dyn Iterator<Item = Result<String>>>  {
        let file = &args.file;
        match file {
            Some(f) => {
                let file = std::fs::File::open(f).unwrap();
                let reader = std::io::BufReader::new(file);
                let lines = reader.lines().map(|l| l.map_err(|e| e.into()));
                Box::new(lines)
            },
            None => Box::new(::std::iter::empty())
        }
    }
}

// provides a reader from a vector of strings
pub struct MemoryReader {
    input: Vec<String>,
}

impl Reader for MemoryReader {
    fn read(&mut self, _: &Args) -> Box<dyn Iterator<Item = Result<String>>> {
        let mut input_lines: Vec<String> = vec![];
        std::mem::swap(&mut input_lines, &mut self.input);
        Box::new(input_lines.into_iter().map(|l| Ok::<String, anyhow::Error>(l)))
    }
}

// a writer than outputs to the standard output/error
pub trait Writer {
    fn write(&mut self, stream: Box<dyn Iterator<Item = Result<String>>>);
}

// writes to to the stdout, putting errors into stderr
pub struct StdoutWriter; 

impl Writer for StdoutWriter {
    fn write(&mut self, stream: Box<dyn Iterator<Item = Result<String>>>) {
       for line in stream {
           match line {
               Ok(l) => println!("{}", l),
               Err(e) => eprintln!("{}", e),
           }
       } 
    }
}

pub struct MemoryWriter {
    output: Vec<String>,
}

