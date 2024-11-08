use clap::Parser;
use colored::Color;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub ignore_case: bool,

    #[clap(short = 'v', long)]
    pub invert_match: bool,

    #[clap(short, long)]
    pub regex: bool,

    #[clap(short, long)]
    pub color: Option<Color>,

    pub needle: String,

    pub file: Option<PathBuf>,
}
