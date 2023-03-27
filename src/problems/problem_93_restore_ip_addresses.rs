//! 93. Restore IP Addresses
//!
//! Medium
//!
//! A valid IP address consists of exactly four integers separated by single dots. Each integer is between 0 and 255 (inclusive) and cannot have leading zeros.
//! For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses, but "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
//! Given a string s containing only digits, return all possible valid IP addresses that can be formed by inserting dots into s.
//! You are not allowed to reorder or remove any digits in s. You may return the valid IP addresses in any order.
//!
//! Example 1:
//! Input: s = "25525511135"
//! Output: ["255.255.11.135","255.255.111.35"]
//!
//! Example 2:
//! Input: s = "0000"
//! Output: ["0.0.0.0"]
//!
//! Example 3:
//! Input: s = "101023"
//! Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
//!
//! Constraints:
//! 1 <= s.length <= 20
//! s consists of digits only.

pub fn get_possible_ip_addresses(s: &str) -> Vec<String> {
    let mut ip_addresses = vec![];
    let mut ip = String::new();

    backtrack(s, &mut ip_addresses, &mut ip, 0, 0);
    ip_addresses
}

fn backtrack(
    text: &str,
    ip_addresses: &mut Vec<String>,
    ip: &mut String,
    num_dots: usize,
    idx: usize,
) {
    if idx == text.len() && num_dots == 3 {
        ip_addresses.push(ip.clone());
        return;
    }

    if num_dots > 3 {
        return;
    }

    for i in idx..text.len() {
        let part = &text[idx..i + 1];

        if is_valid_ip_part(part) {
            let len_ip_before_adding_part = ip.len();

            ip.push_str(part);

            if i + 1 == text.len() {
                backtrack(text, ip_addresses, ip, num_dots, i + 1);
            } else {
                ip.push('.');
                backtrack(text, ip_addresses, ip, num_dots + 1, i + 1);
            }

            *ip = ip[0..len_ip_before_adding_part].to_string();
        }
    }
}

fn is_valid_ip_part(part: &str) -> bool {
    if part.is_empty() || part.len() > 3 {
        return false;
    }

    if part.len() > 1 && part.starts_with('0') {
        return false;
    }

    match part.parse::<usize>() {
        Ok(x) if x > 255 => false,
        Err(_) => false,
        Ok(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_possible_ip_addresses_1() {
        assert_eq!(
            get_possible_ip_addresses("25525511135"),
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()]
        );
    }

    #[test]
    fn test_get_possible_ip_addresses_2() {
        assert_eq!(
            get_possible_ip_addresses("0000"),
            vec!["0.0.0.0".to_string()]
        );
    }

    #[test]
    fn test_get_possible_ip_addresses_3() {
        assert_eq!(
            get_possible_ip_addresses("101023"),
            vec![
                "1.0.10.23".to_string(),
                "1.0.102.3".to_string(),
                "10.1.0.23".to_string(),
                "10.10.2.3".to_string(),
                "101.0.2.3".to_string(),
            ]
        );
    }

    #[test]
    fn test_is_valid_ip_part() {
        for i in 0..=255 {
            assert!(is_valid_ip_part(&i.to_string()));
        }

        assert!(!is_valid_ip_part("256"));
        assert!(!is_valid_ip_part("1222")); // > 3 digits
        assert!(!is_valid_ip_part("01")); // can't start with 0 if part len > 1
    }
}
