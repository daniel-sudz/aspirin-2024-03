
use crate::args::Args;
use anyhow::Result;
use regex::Regex;

// A trait for a preprocessor that performs some transform on the input string
pub trait Transformer {
    fn transform(&self, input: Option<&str>, args: Args) -> Option<String>;
}

// case sensitive preprocessor 
pub struct CaseInsensitivePreprocessor;

impl Transformer for CaseInsensitivePreprocessor {
    fn transform(&self, input: Option<&str>, _: Args) -> Option<String> {
        input.map(|s| s.to_lowercase())
    }
}


// regex match processor

pub struct RegexPreprocessor;

impl Transformer for RegexPreprocessor {
    fn transform(&self, input: Option<&str>, args: Args) -> Option<String> {
        if !input.is_some() {
            return None;
        }

        let re = Regex::new(args.needle.as_str()).unwrap(); 
        let is_match: bool = re.is_match(input.unwrap()) ^ args.invert_match;

        match is_match {
            true => Some(input.unwrap().to_string()),
            false => None,
        }
    }
}

