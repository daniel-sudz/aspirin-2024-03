use anyhow::Result;
use clap::Parser;

mod transformers;
mod args;
mod io;
mod processor;

use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    Ok(())
}
