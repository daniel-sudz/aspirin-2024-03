
use crate::args::Args;
use anyhow::Result;
use colored::Colorize;
use regex::Regex;

// A trait for a preprocessor that performs some transform on the input string
pub trait Transformer {
    fn transform(&self, input: Box<dyn Iterator<Item = Result<String>>>, args: &Args) -> Box<dyn Iterator<Item = Result<String>>>; 
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
        let ignore_case = args.ignore_case;
        let invert_match = args.invert_match;
        match args.regex {
            true => input,
            false => {
                let needle = args.needle.clone();
                Box::new(input.filter(move |s| {
                    match s {
                        Ok(s) => {
                            (match ignore_case {
                                true => s.to_lowercase().contains(&needle.to_lowercase()),
                                false => s.contains(&needle),
                            } ^ invert_match)
                        }
                        Err(_) => false,
                    }
                }))
            }
        }
    }
}

pub struct ColorPreprocessor;

impl Transformer for ColorPreprocessor {
    fn transform(&self, input: Box<dyn Iterator<Item = Result<String>>>, args: &Args) -> Box<dyn Iterator<Item = Result<String>>> {
        let color = args.color.clone();
        match color {
            None => input,
            Some(color) => {
                Box::new(input.map(move |s| {
                    match s {
                        Ok(s) => {
                            Ok(format!("{}", s.color(color)))
                        },
                        Err(e) => Err(e),
                    }
                }))
            }
        }
   }
}