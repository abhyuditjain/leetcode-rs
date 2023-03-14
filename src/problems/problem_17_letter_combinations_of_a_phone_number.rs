//! 17. Letter Combinations of a Phone Number
//!
//! Medium
//!
//! Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
//! Return the answer in any order. A mapping of digits to letters (just like on the telephone buttons) is given below.
//! Note that 1 does not map to any letters.
//! 2 => abc
//! 3 => def
//! 4 => ghi
//! 5 => jkl
//! 6 => mno
//! 7 => pqrs
//! 8 => tuv
//! 9 => wxyz
//!
//! Example 1:
//! Input: digits = "23"
//! Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//!
//! Example 2:
//! Input: digits = ""
//! Output: []
//!
//! Example 3:
//! Input: digits = "2"
//! Output: ["a","b","c"]
//!
//! Constraints:
//! 0 <= digits.length <= 4
//! digits[i] is a digit in the range ['2', '9'].

use std::collections::HashMap;

pub fn letter_combinations(digits: &str) -> Vec<String> {
    let map: HashMap<char, Vec<char>> = vec![
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z']),
    ]
    .into_iter()
    .collect();

    let mut combinations = vec![];
    let mut combination = "".to_owned();
    backtrack(
        &digits.chars().collect(),
        &map,
        0,
        &mut combination,
        &mut combinations,
    );

    combinations
}

fn backtrack(
    digits: &Vec<char>,
    map: &HashMap<char, Vec<char>>,
    i: usize,
    combination: &mut String,
    combinations: &mut Vec<String>,
) {
    if i == digits.len() {
        if !combination.is_empty() {
            combinations.push(combination.clone());
        }
        return;
    }

    for c in map.get(&digits[i]).unwrap() {
        combination.push(*c);
        backtrack(digits, map, i + 1, combination, combinations);
        combination.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations_1() {
        assert_eq!(letter_combinations(""), vec!["".to_owned(); 0]);
    }

    #[test]
    fn test_letter_combinations_2() {
        assert_eq!(
            letter_combinations("2"),
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned(),]
        );
    }

    #[test]
    fn test_letter_combinations_3() {
        assert_eq!(
            letter_combinations("23"),
            vec![
                "ad".to_owned(),
                "ae".to_owned(),
                "af".to_owned(),
                "bd".to_owned(),
                "be".to_owned(),
                "bf".to_owned(),
                "cd".to_owned(),
                "ce".to_owned(),
                "cf".to_owned(),
            ]
        );
    }
}
