mod args;
mod formatter;
mod parser;
mod processor;
mod samples;
use anyhow::Result;
use args::Args;
use clap::Parser;
pub mod maps;

fn main() -> Result<()> {
    let args = Args::parse();
    let output = processor::process_from_string(&args)?;
    println!("{output}");
    Ok(())
}
