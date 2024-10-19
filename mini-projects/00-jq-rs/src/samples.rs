#[cfg(test)]
pub const ALL_TYPES: &str = r#"
        {
            "fizz": "buzz",
            "baz": null,
            "fuzz": true,
            "bizz": 22.0,
            "biz": 42,
            "fizzes": [
                "buzz",
                null,
                true,
                22.0,
                42.0
            ]
        }
    "#;

#[cfg(test)]
pub const ARRAY: &str = r#"
    [
        "one",
        "two",
        "three"
    ]
    "#;
