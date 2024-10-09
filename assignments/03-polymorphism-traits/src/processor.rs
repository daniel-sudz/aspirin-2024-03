use crate::args::Args;
use crate::io;
use crate::transformers::{self, Transformer};

// a factory for creating a processor using abstract base traits
fn processor_factory<'a>(
    reader: &mut Box<dyn io::Reader + 'a>,
    writer: &mut Box<dyn io::Writer + 'a>,
    transformers: Vec<Box<dyn Transformer>>,
    args: Args,
) {
    let mut input = reader.read(&args);

    for transformer in transformers {
        input = transformer.transform(input, &args);
    }
    writer.write(input);
}

pub fn memory_processor_factory<'a>(
    args: Args,
    input: Vec<String>,
    output: &'a mut Vec<String>,
    error: &'a mut Vec<String>,
) {
    let mut reader: Box<dyn io::Reader> = Box::new(io::MemoryReader { input });
    let mut writer: Box<dyn io::Writer<'a> + 'a> = Box::new(io::MemoryWriter { output, error });
    let transformers: Vec<Box<dyn transformers::Transformer>> = vec![
        Box::new(transformers::RegexPreprocessor),
        Box::new(transformers::NeedlePreprocessor),
        Box::new(transformers::ColorPreprocessor),
    ];
    processor_factory(&mut reader, &mut writer, transformers, args);
}

pub fn disk_processor_factory<'a>(args: Args) {
    let mut reader: Box<dyn io::Reader> = Box::new(io::BuffReader);
    let mut writer: Box<dyn io::Writer<'a> + 'a> = Box::new(io::StdoutWriter);
    let transformers: Vec<Box<dyn transformers::Transformer>> = vec![
        Box::new(transformers::RegexPreprocessor),
        Box::new(transformers::NeedlePreprocessor),
        Box::new(transformers::ColorPreprocessor),
    ];
    processor_factory(&mut reader, &mut writer, transformers, args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_basic_processor_test() {
        let mut output: Vec<String> = vec![];
        let mut error: Vec<String> = vec![];
        let input = vec!["Hello, World!".to_string(), "no match".to_string()];
        let args = Args {
            ignore_case: false,
            invert_match: false,
            regex: false,
            color: None,
            needle: "Hello".to_string(),
            file: None,
        };
        memory_processor_factory(args, input, &mut output, &mut error);
        assert_eq!(output, vec!["Hello, World!"]);
        assert_eq!(error, vec![] as Vec<String>);
    }

    #[test]
    fn memory_invert_match_processor_test() {
        let mut output: Vec<String> = vec![];
        let mut error: Vec<String> = vec![];
        let input = vec![
            "Hello, World!".to_string(),
            "no match".to_string(),
            "Hello, World!".to_string(),
            "no match".to_string(),
            "Hello, World!".to_string(),
            "no match".to_string(),
        ];
        let args = Args {
            ignore_case: false,
            invert_match: true,
            regex: false,
            color: None,
            needle: "Hello".to_string(),
            file: None,
        };
        memory_processor_factory(args, input, &mut output, &mut error);
        assert_eq!(output, vec!["no match", "no match", "no match"]);
        assert_eq!(error, vec![] as Vec<String>);
    }

    #[test]
    fn memory_ignore_case_processor_test() {
        let mut output: Vec<String> = vec![];
        let mut error: Vec<String> = vec![];
        let input = vec![
            "Hello, World!".to_string(),
            "no match".to_string(),
            "Hello, World!".to_string(),
            "no match".to_string(),
            "Hello, World!".to_string(),
            "no match".to_string(),
        ];
        let args = Args {
            ignore_case: true,
            invert_match: false,
            regex: false,
            color: None,
            needle: "HeLlO".to_string(),
            file: None,
        };
        memory_processor_factory(args, input, &mut output, &mut error);
        assert_eq!(
            output,
            vec!["Hello, World!", "Hello, World!", "Hello, World!"]
        );
        assert_eq!(error, vec![] as Vec<String>);
    }

    #[test]
    fn memory_color_processor_test() {
        let mut output: Vec<String> = vec![];
        let mut error: Vec<String> = vec![];
        let input = vec!["Hello, World!".to_string()];
        let args = Args {
            ignore_case: true,
            invert_match: false,
            regex: false,
            color: Some(colored::Color::Green),
            needle: "HeLlO".to_string(),
            file: None,
        };
        memory_processor_factory(args, input, &mut output, &mut error);
        assert_eq!(output, vec!["\u{1b}[32mHello, World!\u{1b}[0m"]);
        assert_eq!(error, vec![] as Vec<String>);
    }

    #[test]
    fn memory_regex_processor_test() {
        let mut output: Vec<String> = vec![];
        let mut error: Vec<String> = vec![];
        // from https://docs.rs/regex/latest/regex/#example-find-a-middle-initial
        let input = vec![
            "Homer J. Simpson".to_string(),
            "Homer B. Simpson".to_string(),
            "Foo J. Simpson".to_string(),
            "Bar B. Simpson".to_string(),
        ];
        let args = Args {
            ignore_case: false,
            invert_match: false,
            regex: true,
            color: None,
            needle: r"Homer (.)\. Simpson".to_string(),
            file: None,
        };
        memory_processor_factory(args, input, &mut output, &mut error);
        assert_eq!(output, vec!["Homer J. Simpson", "Homer B. Simpson"]);
        assert_eq!(error, vec![] as Vec<String>);
    }
}
