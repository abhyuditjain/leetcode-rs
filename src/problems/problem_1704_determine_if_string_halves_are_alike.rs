//! 1704. Determine if String Halves Are Alike
//!
//! Easy
//!
//! You are given a string s of even length. Split this string into two halves of equal lengths, and let a be the first half and b be the second half.
//! Two strings are alike if they have the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U').
//! Notice that s contains uppercase and lowercase letters.
//! Return true if a and b are alike. Otherwise, return false.
//!
//! Example 1:
//! Input: s = "book"
//! Output: true
//! Explanation: a = "bo" and b = "ok". a has 1 vowel and b has 1 vowel. Therefore, they are alike.
//!
//! Example 2:
//! Input: s = "textbook"
//! Output: false
//! Explanation: a = "text" and b = "book". a has 1 vowel whereas b has 2. Therefore, they are not alike.
//! Notice that the vowel o is counted twice.
//!
//! Constraints:
//! 2 <= s.length <= 1000
//! s.length is even.
//! s consists of uppercase and lowercase letters.

pub fn are_halves_alike(s: &str) -> bool {
    // check constraints
    assert!(s.len() > 1 && s.len() % 2 == 0);

    let n = s.len();

    let count = s[..n / 2]
        .chars()
        .zip(s[n / 2..].chars())
        .fold(0, |count, (x, y)| match (is_vowel(x), is_vowel(y)) {
            (true, true) => count,
            (true, false) => count + 1,
            (false, true) => count - 1,
            (false, false) => count,
        });

    count == 0
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_halves_alike() {
        assert!(are_halves_alike("book"));
        assert!(!are_halves_alike("textbook"));
    }
}
