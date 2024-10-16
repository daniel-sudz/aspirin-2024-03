use crate::maps;
use crate::maps::maps::Map;
use crate::maps::object_identifier::ObjectIdentifierMap;
use crate::maps::array_iterator::ArrayIteratorMap;
use crate::maps::identity::IdentityMap;
use crate::maps::add::AddMap;
use crate::maps::del::DelMap;
use crate::maps::length::LengthMap;
use anyhow::Result;
use regex::Regex;

fn array_slice_match(input: &str) -> Result<Box<dyn Map>> {
   
}

fn del_slice_match(input: &str) -> Result<Box<dyn Map>> {
   }

fn parse_sub(input: String) -> Result<Box<dyn Map>> {
    let input = input.trim();
    let input_chars = input.chars().collect::<Vec<char>>();

    // direct matches
    if input == "." {
        return Ok(Box::new(IdentityMap));
    } 
    if input == "add" {
        return Ok(Box::new(AddMap));
    }
    else if input == "length" {
        return Ok(Box::new(LengthMap));
    }
    else if input == ".[]" {
        return Ok(Box::new(ArrayIteratorMap));
    }

    // more complex patterns
    if input_chars.starts_with(".") {
        if input_chars[1] == '[' {
            if input_chars[input_chars.len() - 1] == ']' {
            }
            else {
                return anyhow::anyhow!("failed to parse array iterator");
            }
            return Ok(Box::new(ArrayIteratorMap));
        }
    }

}
pub fn parse(input: &str) -> Result<Vec<Box<dyn Map>>> {
    let mut stack: Vec<String> = Vec::new();

    for i in input.chars() {

    Ok(Vec::new())
}
*/

mod tests {
    use super::*;

    #[test]
    fn test_identity_map() {
        let pattern = r"\.\[(\d+):(\d+)\]";
        let re = Regex::new(pattern).expect("Invalid regex pattern");

        if let Some(captures) = re.captures(".[0]") {
            let first = captures.get(0).unwrap().as_str();
            let second = captures.get(1).unwrap().as_str();
            let third = captures.get(2).unwrap().as_str();
            println!("first: {}", first);
            println!("second: {}", second);
            println!("third: {}", third);
        }
    }
}