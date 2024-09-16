#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::And => write!(f, "&"),
            Operation::Or => write!(f, "|"),
            Operation::Xor => write!(f, "^"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Base {
    Binary = 2,
    Decimal = 10,
    Hexadecimal = 16,
}

fn parse_base(num: &str) -> Option<Base> {
    match num.len() {
        0 => None,
        1 => match num.chars().all(|c| c.is_ascii_digit()) {
            true => Some(Base::Decimal),
            false => None,
        },
        _ => match &num[0..2] {
            "0b" => Some(Base::Binary),
            "0x" => Some(Base::Hexadecimal),
            _ => match num.chars().all(|c| c.is_ascii_digit()) {
                true => Some(Base::Decimal),
                false => None,
            },
        },
    }
}

fn parse_num(num: &str) -> Option<u32> {
    let base: Option<Base> = parse_base(num);
    match base {
        None => None,
        Some(_) => {
            let radix_parse = match base.unwrap() {
                Base::Binary | Base::Hexadecimal => {
                    u32::from_str_radix(&num[2..], base.unwrap() as u32)
                }
                Base::Decimal => u32::from_str_radix(num, base.unwrap() as u32),
            };
            match radix_parse {
                Ok(n) => Some(n),
                Err(_) => None,
            }
        }
    }
}

fn parse_operation(op: &str) -> Option<Operation> {
    match op.to_ascii_uppercase().as_str() {
        "AND" | "&" => Some(Operation::And),
        "OR" | "|" => Some(Operation::Or),
        "XOR" | "^" => Some(Operation::Xor),
        _ => None,
    }
}

fn evaluate(op: Operation, num1: u32, num2: u32) -> u32 {
    match op {
        Operation::And => num1 & num2,
        Operation::Or => num1 | num2,
        Operation::Xor => num1 ^ num2,
    }
}

fn evaluate_input(op: &str, num1: &str, num2: &str) -> String {
    let op: Option<Operation> = parse_operation(op);
    let num1: Option<u32> = parse_num(num1);
    let num2: Option<u32> = parse_num(num2);
    match (op, num1, num2) {
        (Some(op), Some(num1), Some(num2)) => {
            let result = evaluate(op, num1, num2);
            format!("The result of {num1} {op} {num2} is {result}")
        }
        (_, None, _) => "Invalid first number".to_string(),
        (_, _, None) => "Invalid second number".to_string(),
        (None, _, _) => "Invalid operation".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::calculator::{
        evaluate_input, parse_base, parse_num, parse_operation, Base, Operation,
    };

    #[test_case("0x10", Some(Base::Hexadecimal); "parse base test 1")]
    #[test_case("10", Some(Base::Decimal); "parse base test 2")]
    #[test_case("0b10", Some(Base::Binary); "parse base test 3")]
    #[test_case("z", None; "parse base test 4")]
    #[test_case("zz", None; "parse base test 5")]
    fn test_parse_base(base: &str, expected: Option<Base>) {
        assert_eq!(parse_base(base), expected);
    }

    #[test_case("0x10", Some(16); "parse num test 1")]
    #[test_case("16", Some(16); "parse num test 2")]
    #[test_case("0b10000", Some(16); "parse num test 3")]
    #[test_case("z", None; "parse num test 4")]
    #[test_case("zz", None; "parse num test 5")]
    fn test_parse_number(num: &str, expected: Option<u32>) {
        assert_eq!(parse_num(num), expected);
    }

    #[test_case("|", Some(Operation::Or); "parse op test 1")]
    #[test_case("&", Some(Operation::And); "parse op test 2")]
    #[test_case("^", Some(Operation::Xor); "parse op test 3")]
    #[test_case("OR", Some(Operation::Or); "parse op test 4")]
    #[test_case("AND", Some(Operation::And); "parse op test 5")]
    #[test_case("XOR", Some(Operation::Xor); "parse op test 6")]
    #[test_case("z", None; "parse op test 7")]
    #[test_case("zz", None; "parse op test 8")]
    fn test_parse_op(op: &str, expected: Option<Operation>) {
        assert_eq!(parse_operation(op), expected);
    }

    #[test_case("AND", "0b1010", "0b1100", "The result of 10 & 12 is 8"; "evaluate binary test 1")]
    #[test_case("OR", "0b1010", "0b1100", "The result of 10 | 12 is 14"; "evaluate binary test 2")]
    #[test_case("XOR", "0b1010", "0b1100", "The result of 10 ^ 12 is 6"; "evaluate binary test 3")]
    #[test_case("FOO", "0b1010", "0b1100", "Invalid operation"; "evaluate binary test 4")]
    #[test_case("AND", "10", "12", "The result of 10 & 12 is 8"; "evaluate decimal test 1")]
    #[test_case("OR", "10", "12", "The result of 10 | 12 is 14"; "evaluate decimal test 2")]
    #[test_case("XOR", "10", "12", "The result of 10 ^ 12 is 6"; "evaluate decimal test 3")]
    #[test_case("FOO", "10", "12", "Invalid operation"; "evaluate decimal test 4")]
    #[test_case("AND", "0xA", "0xC", "The result of 10 & 12 is 8"; "evaluate hex test 1")]
    #[test_case("OR", "0xA", "0xC", "The result of 10 | 12 is 14"; "evaluate hex test 2")]
    #[test_case("XOR", "0xA", "0xC", "The result of 10 ^ 12 is 6"; "evaluate hex test 3")]
    #[test_case("FOO", "0xA", "0xC", "Invalid operation"; "evaluate hex test 4")]
    fn test_evaluate_input(op: &str, num1: &str, num2: &str, result: &str) {
        assert_eq!(evaluate_input(op, num1, num2), result);
    }
}
