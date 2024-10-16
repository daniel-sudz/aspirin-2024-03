mod args;
mod parser;
use args::Args;
use clap::Parser;

pub mod maps;

fn main() {
    let args = Args::parse();
    println!("Hello World!");
}
