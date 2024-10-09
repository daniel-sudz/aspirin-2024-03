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


pub fn memory_processor_factory<'a>(args: Args, input: Vec<String>, output: &'a mut Vec<String>, error: &'a mut Vec<String>) {
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
