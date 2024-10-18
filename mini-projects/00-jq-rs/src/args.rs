use std::path::PathBuf;
use anyhow::Result;
use serde_json::Value;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long)]
    pub color_output: Option<bool>,

    #[clap(long)]
    pub monochrome_output: Option<bool>,

    #[clap(long)]
    pub sort_keys: Option<bool>,

    #[clap(long)]
    pub indent: Option<u8>,

    #[clap(long)]
    pub compact_output: Option<bool>,

    pub file: Option<PathBuf>,

}

impl Args {
    pub fn validate(&self) -> Result<()> {
        match (self.color_output, self.monochrome_output) {
            (Some(true), Some(true)) | (Some(false), Some(false)) => anyhow::bail!("cannot specify both color and monochrome output"),
            _ => () 
        }
        match (self.compact_output, self.indent) {
            (Some(true), Some(_)) | (Some(false), Some(_)) => anyhow::bail!("cannot specify both compact and indent output"),
            _ => () 
        }
        Ok(())
    }

    pub fn is_color_output(&self) -> Result<bool> {
        match self.color_output {
            Some(true) => Ok(true),
            Some(false) => Ok(false),
            None => Ok(false)
        }
    }

    pub fn is_sort_keys(&self) -> Result<bool> {
        match self.sort_keys {
            Some(true) => Ok(true),
            Some(false) => Ok(false),
            None => Ok(false)
        }
    }

    pub fn is_compact_output(&self) -> Result<bool> {
        match self.compact_output {
            Some(true) => Ok(true),
            Some(false) => Ok(false),
            None => Ok(false)
        }
    }

    pub fn get_indent(&self) -> Result<u8> {
        match self.indent {
            Some(x) => Ok(x),
            None => Ok(2)
        }
    }

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