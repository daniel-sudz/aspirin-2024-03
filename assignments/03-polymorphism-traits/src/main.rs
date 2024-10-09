use anyhow::Result;
use clap::Parser;

pub mod transformers;
pub mod args;
pub mod io;
pub mod processor;

use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    processor::disk_processor_factory(args);
    Ok(())
}
