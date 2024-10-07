use crate::args::Args;
use crate::transformers::{self, Transformer};
use crate::io;

// a factory for creating a processor using abstract base traits
fn processor_factory(
    reader: Box<dyn io::Reader>, 
    writer: Box<dyn io::Writer>,
    transformers: Vec<Box<dyn Transformer>>,
    args: Args 
) {
    let mut input = reader.read(&args);

    for transformer in transformers {
        input = transformer.transform(input, &args);
    }
    writer.write(input);
}


fn memory_processor_factory(args: Args) {
    let reader = Box::new(io::BuffReader);
    let writer = Box::new(io::StdoutWriter);

    let transformers: Vec<Box<dyn transformers::Transformer>> = vec![
        Box::new(transformers::CaseInsensitivePreprocessor),
        Box::new(transformers::RegexPreprocessor),
    ];
    processor_factory(reader, writer, transformers, args);
}