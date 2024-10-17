use serde_json::Value;

pub fn format(input: Value) -> String {
    serde_json::to_string(&input).unwrap()
}

mod tests {
    
}