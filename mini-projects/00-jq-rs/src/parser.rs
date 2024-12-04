use crate::maps::add::AddMap;
use crate::maps::array_index::ArrayIndexMap;
use crate::maps::array_iterator::ArrayIteratorMap;
use crate::maps::array_slice::ArraySliceMap;
use crate::maps::del_array::DelMapArray;
use crate::maps::del_key::DelMapKey;
use crate::maps::identity::IdentityMap;
use crate::maps::length::LengthMap;
use crate::maps::object_identifier::ObjectIdentifierMap;
use crate::maps::Map;
use anyhow::Result;

// parses the jq command into a list of maps
// maps are split by the pipe operator and then matched against the available maps
// maps use a regex to match the command and then parse the arguments
pub fn parse(input: String) -> Result<Vec<Box<dyn Map>>> {
    let maps: Vec<Box<dyn Map>> = vec![
        Box::new(DelMapKey {
            key: "".to_string(),
        }),
        Box::new(DelMapArray { from: 0, to: 0 }),
        Box::new(ArraySliceMap { from: 0, to: 0 }),
        Box::new(ArrayIndexMap { index: 0 }),
        Box::new(ArrayIteratorMap {}),
        Box::new(ObjectIdentifierMap {
            key: "".to_string(),
        }),
        Box::new(AddMap {}),
        Box::new(LengthMap {}),
        Box::new(IdentityMap),
    ];
    let ops: Result<Vec<Box<dyn Map>>> = input
        .split("|")
        .map(|op| {
            for map in maps.iter() {
                if let Ok(map) = map.command_match(op.to_string().trim()) {
                    return Ok(map);
                }
            }
            anyhow::bail!("failed to parse command: {}", op);
        })
        .collect();
    ops
}

// the parser is tested implicitly in the integration cli test so no additional tests are needed
#[cfg(test)]
mod tests {}
