use std::collections::HashMap;
use std::cmp::max;

fn longest_equal_sequence_prescriptive<T: std::cmp::PartialOrd>(sequence: &[T]) -> i32 {
    if sequence.len() == 0 {
        return 0;
    }
    let mut ans: i32 = 0;
    let mut cur_ans: i32 = 1;
    let mut last = &sequence[0];
    for i in 1..sequence.len() {
        if sequence[i] == *last {
            cur_ans += 1;
        }
        else {
            ans = max(ans, cur_ans);
            cur_ans = 1;
            last = &sequence[i];
        }
    }
    max(ans, cur_ans)
}

fn longest_equal_sequence_functional<T: std::cmp::PartialOrd>(sequence: &[T]) -> i32 {
    match sequence.len() {
        0 => 0,
        _ => sequence.into_iter().fold((0,0,&sequence[0]), |(ans, cur_ans, last), x| {
            match last == x {
                true => (max(ans,cur_ans+1), cur_ans+1, last),
                false => (max(ans,cur_ans), 1, x)
            }
        }).0
    }
}

fn is_valid_paranthesis(paranthesis: &str) -> bool {
    paranthesis.chars().into_iter().fold((0,0,0,true), |s, c| {
        let next = match c {
            '(' => (s.0+1, s.1, s.2),
            ')' => (s.0-1, s.1, s.2),
            '{' => (s.0, s.1+1, s.2),
            '}' => (s.0, s.1-1, s.2),
            '[' => (s.0, s.1, s.2+1),
            ']' => (s.0, s.1, s.2-1),
            _ => (-1,-1,-1)
        };
        match (next.0 >= 0, next.1 >= 0, next.2 >= 0, s.3) {
            (true,true,true,true) => (next.0, next.1, next.2, true),
            (_,_,_,_) => (-1,-1,-1,true)
        }
    }).eq(&(0,0,0,true))
}

fn longest_common_substring<'a>(first_str: &'a str, second_str: &str) -> &'a str {
    let len_first = first_str.chars().count();
    let len_second = second_str.chars().count();

    let first_chars: Vec<char> = first_str.chars().collect();
    let second_chars: Vec<char> = second_str.chars().collect();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; len_second]; len_first];

    for i in 0..len_first {
        for j in 0..len_second {
            if first_chars[i] == second_chars[j] {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                }
                else {
                    dp[i][j] = dp[i-1][j-1] + 1;
                }
            }
        }
    }

    let mut len_match = 0;
    let mut ans: &str = &"";
    for i in 0..len_first {
        for j in 0..len_second {
            if dp[i][j] > len_match {
                len_match = dp[i][j];
                let start = first_str.char_indices().nth((i+1) - len_match as usize).unwrap().0;
                let end = first_str.char_indices().nth(i).unwrap().0;
                ans = &first_str[start..end+1];
            }
        }
    }
    ans
}

fn longest_common_substring_multiple<'a>(strings: &[&'a str]) -> &'a str {
    let mut ans: &str = &strings[0];
    for i in 1..strings.len() {
        ans = longest_common_substring(ans, &strings[i]);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_equal_sequence_prescriptive() {
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_prescriptive(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_prescriptive(&vec), 3);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }
    #[test]
    fn test_longest_equal_sequence_functional() {
        assert_eq!(longest_equal_sequence_functional(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_functional(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_functional(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_functional(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_functional(&vec), 3);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }

    #[test]
    fn test_is_valid_paranthesis() {
        assert_eq!(is_valid_paranthesis(&String::from("{}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()[]{}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("({[]})")), true);
        assert_eq!(is_valid_paranthesis(&String::from("([]){}{}([]){}")), true);
        assert_eq!(is_valid_paranthesis(&String::from("()(")), false);
        assert_eq!(is_valid_paranthesis(&String::from("(()")), false);
        assert_eq!(is_valid_paranthesis(&String::from("([)]{[})")), false);
        assert_eq!(
            is_valid_paranthesis(&String::from("({[()]}){[([)]}")),
            false
        );
        assert_eq!(
            is_valid_paranthesis(&String::from("()[]{}(([])){[()]}(")),
            false
        );
    }

    #[test]
    fn test_common_substring() {
        assert_eq!(longest_common_substring(&"abcdefg", &"bcdef"), "bcdef");
        assert_eq!(longest_common_substring(&"apple", &"pineapple"), "apple");
        assert_eq!(longest_common_substring(&"dog", &"cat"), "");
        assert_eq!(longest_common_substring(&"racecar", &"racecar"), "racecar");
        assert_eq!(longest_common_substring(&"ababc", &"babca"), "babc");
        assert_eq!(longest_common_substring(&"xyzabcxyz", &"abc"), "abc");
        assert_eq!(longest_common_substring(&"", &"abc"), "");
        assert_eq!(longest_common_substring(&"abcdefgh", &"defghijk"), "defgh");
        assert_eq!(longest_common_substring(&"xyabcz", &"abcxy"), "abc");
        assert_eq!(longest_common_substring(&"ABCDEFG", &"abcdefg"), "");
        assert_eq!(
            longest_common_substring(
                &"thisisaverylongstringwithacommonsubstring",
                &"anotherlongstringwithacommonsubstring"
            ),
            "longstringwithacommonsubstring"
        );
        assert_eq!(longest_common_substring("a", "a"), "a");
    }

    #[test]
    fn test_common_substring_multiple() {
        assert_eq!(
            longest_common_substring_multiple(&vec!["abcdefg", "cdef"]),
            "cdef"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["apple", "pineapple", "maple", "snapple"]),
            "ple"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["dog", "cat", "fish"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["racecar", "car", "scar"]),
            "car"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["ababc", "babca", "abcab"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyzabcxyz", "abc", "zabcy", "abc"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["", "abc", "def"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "abcdefgh",
                "bcd",
                "bcdtravels",
                "abcs",
                "webcam"
            ]),
            "bc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["identical", "identical", "identical"]),
            "identical"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyabcz", "abcxy", "zabc"]),
            "abc"
        );
        assert_eq!(longest_common_substring_multiple(&vec!["a", "a", "a"]), "a");
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "thisisaverylongstringwiththecommonsubstring",
                "anotherlongstringwithacommonsubstring",
                "yetanotherstringthatcontainsacommonsubstring",
            ]),
            "commonsubstring",
        );
    }
}
