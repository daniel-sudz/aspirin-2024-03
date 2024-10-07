use std::io::BufRead;
use anyhow::Result;
use crate::args::Args;

// a reader than takes in the input and outputs an iterator of lines
pub trait Reader {
    fn read(&self, args: Args) -> Box<dyn Iterator<Item = Result<String>>>;
}

// case sensitive preprocessor 
pub struct BuffReader;

// buffered reader from disk
impl Reader for BuffReader {
    fn read(&self, args: Args) -> Box<dyn Iterator<Item = Result<String>>>  {
        let file = args.file.unwrap();
        let file = std::fs::File::open(file).unwrap();
        let reader = std::io::BufReader::new(file);
        let lines = reader.lines().map(|l| l.map_err(|e| e.into()));
        Box::new(lines)
    }
}


// a writer than outputs to the standard output/error
pub trait Writer {
    fn write(&self, stream: Box<dyn Iterator<Item = Result<String>>>);
}

// writes to to the stdout, putting errors into stderr
pub struct StdoutWriter; 

impl Writer for StdoutWriter {
    fn write(&self, stream: Box<dyn Iterator<Item = Result<String>>>) {
       for line in stream {
           match line {
               Ok(l) => println!("{}", l),
               Err(e) => eprintln!("{}", e),
           }
       } 
    }
}