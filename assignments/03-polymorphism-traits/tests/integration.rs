
use greprs::processor::memory_processor_factory;
use greprs::args::Args;
use colored::Color;

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
    assert_eq!(output, vec!["Hello, World!", "Hello, World!", "Hello, World!"]);
    assert_eq!(error, vec![] as Vec<String>);
}

#[test]
fn memory_color_processor_test() {
    let mut output: Vec<String> = vec![];
    let mut error: Vec<String> = vec![];
    let input = vec![
        "Hello, World!".to_string(),
    ];
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