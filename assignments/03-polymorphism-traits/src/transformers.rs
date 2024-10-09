
use crate::args::Args;
use anyhow::Result;
use regex::Regex;

// A trait for a preprocessor that performs some transform on the input string
pub trait Transformer {
    fn transform(&self, input: Box<dyn Iterator<Item = Result<String>>>, args: &Args) -> Box<dyn Iterator<Item = Result<String>>>; 
}

// case sensitive preprocessor 
pub struct CaseInsensitivePreprocessor;

impl Transformer for CaseInsensitivePreprocessor {
    fn transform(&self, input: Box<dyn Iterator<Item = Result<String>>>, _: &Args) -> Box<dyn Iterator<Item = Result<String>>> {
        Box::new(input.map(|s| Ok(s?.to_lowercase())))
    }
}


// regex match processor

pub struct RegexPreprocessor;

impl Transformer for RegexPreprocessor {
    fn transform(&self, input: Box<dyn Iterator<Item = Result<String>>>, args: &Args) -> Box<dyn Iterator<Item = Result<String>>> {
        match args.regex {
            false => input,
            true => {
                let re = Regex::new(args.needle.as_str()).unwrap(); 
                Box::new(input.filter(move |s| {
                    match s {
                        Ok(s) => re.is_match(s),
                        Err(_) => false,
                    }
                }))
            }
        }
    }
}

pub struct NeedlePreprocessor; 

impl Transformer for NeedlePreprocessor {
    fn transform(&self, input: Box<dyn Iterator<Item = Result<String>>>, args: &Args) -> Box<dyn Iterator<Item = Result<String>>> {
        match args.regex {
            true => input,
            false => {
                let needle = args.needle.clone();
                Box::new(input.filter(move |s| {
                    match s {
                        Ok(s) => s.contains(&needle),
                        Err(_) => false,
                    }
                }))
            }
        }
    }
}