use crate::maps;
use crate::maps::maps::Map;
use crate::maps::object_identifier::ObjectIdentifierMap;
use crate::maps::array_slice::ArraySliceMap;
use crate::maps::identity::IdentityMap;
use crate::maps::add::AddMap;
use crate::maps::del_key::DelMapKey;
use crate::maps::length::LengthMap;
use crate::maps::array_index::ArrayIndexMap;
use anyhow::Result;

pub fn parse(input: String) -> Result<Vec<Box<dyn Map>>> {
    let maps: Vec<Box<dyn Map>> = vec![
        Box::new(ObjectIdentifierMap { key: "".to_string() }),
        Box::new(IdentityMap),
        Box::new(AddMap {}),
        Box::new(DelMapKey { key: "".to_string() }),
        Box::new(LengthMap {}),
        Box::new(ArraySliceMap { from: 0, to: 0 }),
        Box::new(ArrayIndexMap { index: 0 }),

    ];
    let ops: Result<Vec<Box<dyn Map>>> = input.split("|").map(|op| {
        for map in maps.iter() {
            if let Ok(map) = map.command_match(op.to_string().trim()) {
                return Ok(map);
            }
        }
        anyhow::bail!("failed to parse command: {}", op);
    }).collect();
    ops
}

mod tests {
    use super::*;
}