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

    pub file: String,

}