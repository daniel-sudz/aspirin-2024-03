use std::path::PathBuf;
use anyhow::Result;
use serde_json::Value;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, action, conflicts_with = "monochrome_output")]
    pub color_output: bool,

    #[clap(long, action, conflicts_with = "color_output")]
    pub monochrome_output: bool,

    #[clap(long, action)]
    pub sort_keys: bool,

    #[clap(long, conflicts_with = "compact_output")]
    pub indent: Option<u8>,

    #[clap(long, action, conflicts_with = "indent")]
    pub compact_output: bool,

    #[clap(required = true)]
    pub command: String,

    pub file: Option<PathBuf>,
}

impl Args {
    pub fn get_input(&self) -> Result<Value> {
        match &self.file {
            Some(f) => {
                let file = std::fs::File::open(f)?;
                let reader = std::io::BufReader::new(file);
                let v: Value = serde_json::from_reader(reader)?;
                Ok(v)
            }
            None => {
                let stdin = std::io::stdin();
                let reader = std::io::BufReader::new(stdin);
                let v: Value = serde_json::from_reader(reader)?;
                Ok(v)
            }
        }
    }
}