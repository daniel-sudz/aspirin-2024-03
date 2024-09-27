use std::iter::once;
use std::collections::HashSet;
use std::mem::swap;

fn split_string<'a>(string: &'a str, delimeter: &str) -> Vec<&'a str> {
    let result: Vec<&str> = string.split(delimeter).collect();
    result
}

#[derive(PartialEq, Debug)]
struct  Differences<'a> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'a str>,
}

fn find_differences<'a>(first_string: &'a str, second_string: &'a str) -> Differences<'a> {
    let split_first = split_string(first_string, " ");
    let split_second = split_string(second_string, " ");

    let mut first_set: HashSet<&str> = HashSet::new();
    let mut second_set: HashSet<&str> = HashSet::new();

    for word in &split_first {
        first_set.insert(word);
    }

    for word in &split_second {
        second_set.insert(word);
    }

    let mut only_in_first: Vec<&'a str> = Vec::new();
    let mut only_in_second: Vec<&'a str> = Vec::new(); 

    for word in &split_first {
        if !second_set.contains(word) {
            only_in_first.push(word);
        }
    }

    for word in &split_second {
        if !first_set.contains(word) {
            only_in_second.push(word);
        }
    }

    Differences {
        only_in_first,
        only_in_second,
    }
}


fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn merge_names(first_name: &str, second_name: &str) -> String {
    let mut merged_name = String::new();

    let mut iter_first = first_name.chars().peekable();
    let mut iter_second = second_name.chars().peekable();

    while iter_first.peek().is_some() || iter_second.peek().is_some() {
        let first_char = iter_first.next();
        if first_char.is_some() && is_vowel(first_char.unwrap()) {
            merged_name.push(first_char.unwrap());
        }
        
        let mut head_char = iter_first.next();
        while head_char.is_some() && !is_vowel(head_char.unwrap()) {
            merged_name.push(head_char.unwrap());
            head_char = iter_first.next();
        }

        swap(&mut iter_first, &mut iter_second);
    }

    merged_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string(&"", &""), Vec::<&str>::new());
        assert_eq!(
            split_string(&"Hello, World!", &", "),
            vec!["Hello", "World!"]
        );
        assert_eq!(
            split_string(
                &"I this think this that this sentence this is this very this confusing this ",
                &" this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string(&"apple🍎banana🍎orange", &"🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string(
                &"Ayush;put|a,lot~of`random;delimeters|in|this,sentence",
                &";"
            ),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences(&"", &""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(&"pineapple pen", &"apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(
                &"Sally sold seashells at the seashore",
                &"Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                &"How much ground would a groundhog hog",
                &"If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names(&"alex", &"jake"), "aljexake");
        assert_eq!(merge_names(&"steven", &"stephen"), "ststevephenen");
        assert_eq!(merge_names(&"gym", &"rhythm"), "gymrhythm");
        assert_eq!(merge_names(&"walter", &"gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names(&"baker", &"quaker"), "bqakueraker");
        assert_eq!(merge_names(&"", &""), "");
        assert_eq!(merge_names(&"samesies", &"samesies"), "ssamamesesiieses");
        assert_eq!(merge_names(&"heather", &"meagan"), "hmeeathageran");
        assert_eq!(merge_names(&"panda", &"turtle"), "ptandurtlae");
        assert_eq!(merge_names(&"hot", &"sauce"), "hsotauce");
        assert_eq!(merge_names(&"", &"second"), "second");
        assert_eq!(merge_names(&"first", &""), "first");
    }
}
