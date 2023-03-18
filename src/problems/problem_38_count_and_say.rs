//! 38. Count and Say
//!
//! Medium
//!
//! The count-and-say sequence is a sequence of digit strings defined by the recursive formula:
//! countAndSay(1) = "1"
//! countAndSay(n) is the way you would "say" the digit string from countAndSay(n-1), which is then converted into a different digit string.
//! To determine how you "say" a digit string, split it into the minimal number of substrings such that each substring contains exactly one unique digit. Then for each substring, say the number of digits, then say the digit. Finally, concatenate every said digit.
//! For example, the saying and conversion for digit string "3322251":
//! Given a positive integer n, return the nth term of the count-and-say sequence.
//!
//! Example 1:
//! Input: n = 1
//! Output: "1"
//! Explanation: This is the base case.
//!
//! Example 2:
//! Input: n = 4
//! Output: "1211"
//! Explanation:
//! countAndSay(1) = "1"
//! countAndSay(2) = say "1" = one 1 = "11"
//! countAndSay(3) = say "11" = two 1's = "21"
//! countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
//!
//! Constraints:
//! 1 <= n <= 30

pub fn count_and_say(n: i32) -> String {
    let mut s = "".to_string();

    for _ in 0..n {
        s = say(s.as_bytes());
    }

    s
}

fn say(s: &[u8]) -> String {
    if s.is_empty() {
        return "1".to_string();
    }

    let mut res = String::new();
    let mut count = 1;

    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            count += 1;
        } else {
            res.push_str(count.to_string().as_str());
            res.push(s[i - 1] as char);
            count = 1;
        }
    }

    res.push_str(count.to_string().as_str());
    res.push(*s.last().unwrap() as char);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        assert_eq!(count_and_say(1), "1".to_string());
        assert_eq!(count_and_say(4), "1211".to_string());
        assert_eq!(count_and_say(5), "111221".to_string());
    }
}
