
use crate::args::Args;
use anyhow::Result;
use colored::Colorize;
use regex::Regex;

// A trait for a preprocessor that performs some transform on the input string
pub trait Transformer {
    fn transform(&self, input: Box<dyn Iterator<Item = Result<String>>>, args: &Args) -> Box<dyn Iterator<Item = Result<String>>>; 
}

// regex match processor
#[derive(Default)]
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

#[derive(Default)]
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

#[derive(Default)]
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

mod test {
    use super::*;

    #[test]
    fn test_needle_processor() {
        let input: Vec<Result<String, anyhow::Error>> = vec![
            Ok("Hello, World!".to_string()),
            Ok("no match".to_string()),
            Ok("Hello, World!".to_string()),
            Ok("no match".to_string()),
            Ok("Hello, World!".to_string()),
            Ok("no match".to_string()),
        ];

        let args = Args {
            ignore_case: false,
            invert_match: false,
            regex: false,
            color: None,
            needle: "Hello".to_string(),
            file: None,
        };

        let needle_processor = NeedlePreprocessor;
        let res = needle_processor.transform(Box::new(input.into_iter()), &args);
        let res_vec: Vec<String> = res.map(|x| {
            match x {
                Ok(x) => x,
                Err(x) => x.to_string()
            }
        }).collect();

        assert_eq!(res_vec, vec!["Hello, World!", "Hello, World!", "Hello, World!"]);
    }
}