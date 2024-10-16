use crate::maps;
use crate::maps::maps::Map;
use crate::maps::object_identifier::ObjectIdentifierMap;
use crate::maps::array_slice::ArrayIteratorMap;
use crate::maps::identity::IdentityMap;
use crate::maps::add::AddMap;
use crate::maps::del::DelMap;
use crate::maps::length::LengthMap;
use anyhow::Result;
use regex::Regex;

/* 
pub fn parse(input: String) -> Result<Vec<Box<dyn Map>>> {
    let mut maps: Vec<Box<dyn Map>> = vec![
        ObjectIdentifierMap { key: "".to_string() }
    ];
    let splits = input.split("|").collect::<Vec<&str>>();

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