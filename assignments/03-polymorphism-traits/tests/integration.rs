
use greprs::processor::memory_processor_factory;
use greprs::args::Args;

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
fn memory_ignore_case_preprocessor_test() {
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
    assert_eq!(output, vec!["Hello, World!", "Hello, World!", "Hello, World!"]);
    assert_eq!(error, vec![] as Vec<String>);
}