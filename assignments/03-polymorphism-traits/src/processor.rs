use crate::args::Args;
use crate::transformers::{self, Transformer};
use crate::io;

// a factory for creating a processor using abstract base traits
fn processor_factory<'a>(
    reader: &mut Box<dyn io::Reader + 'a>, 
    writer: &mut Box<dyn io::Writer + 'a>,
    transformers: Vec<Box<dyn Transformer>>,
    args: Args 
) {
    let mut input = reader.read(&args);

    for transformer in transformers {
        input = transformer.transform(input, &args);
    }
    writer.write(input);
}


fn memory_processor_factory<'a>(args: Args, input: Vec<String>, output: &'a mut Vec<String>, error: &'a mut Vec<String>) {
    let mut reader: Box<dyn io::Reader> = Box::new(io::MemoryReader {input});
    let mut writer: Box<dyn io::Writer<'a> + 'a> = Box::new(io::MemoryWriter {
        output,
        error,
    });
    let transformers: Vec<Box<dyn transformers::Transformer>> = vec![
        Box::new(transformers::CaseInsensitivePreprocessor),
        Box::new(transformers::RegexPreprocessor),
        Box::new(transformers::NeedlePreprocessor)
    ];
    processor_factory(&mut reader, &mut writer, transformers, args);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = 4;
        assert_eq!(result, 4);
    }

    #[test]
    fn basic_memory_processor() {
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

}