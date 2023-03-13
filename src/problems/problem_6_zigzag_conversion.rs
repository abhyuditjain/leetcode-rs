//! 6. Zigzag Conversion
//!
//! Medium
//!
//! The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
//! P   A   H   N
//! A P L S I I G
//! Y   I   R
//! And then read line by line: "PAHNAPLSIIGYIR"
//!
//! Write the code that will take a string and make this conversion given a number of rows:
//! string convert(string s, int numRows);
//!
//! Example 1:
//! Input: s = "PAYPALISHIRING", numRows = 3
//! Output: "PAHNAPLSIIGYIR"
//!
//! Example 2:
//! Input: s = "PAYPALISHIRING", numRows = 4
//! Output: "PINALSIGYAHRPI"
//! Explanation:
//! P     I    N
//! A   L S  I G
//! Y A   H R
//! P     I
//!
//! Example 3:
//! Input: s = "A", numRows = 1
//! Output: "A"

//! Constraints:
//! 1 <= s.length <= 1000
//! s consists of English letters (lower-case and upper-case), ',' and '.'.
//! 1 <= numRows <= 1000

pub fn zigzag_convert(s: &str, num_rows: usize) -> String {
    use Direction::*;

    if num_rows == 1 {
        return s.to_owned();
    }

    let mut strings = vec!["".to_owned(); num_rows];

    let mut step = Down;
    let mut curr_row = 0;

    for ch in s.chars() {
        strings[curr_row as usize].push(ch);

        let step_size: i32 = step.into();
        curr_row += step_size;

        if curr_row as usize == num_rows - 1 {
            step = Up;
        } else if curr_row == 0 {
            step = Down;
        }
    }

    strings.join("")
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

impl From<Direction> for i32 {
    fn from(value: Direction) -> Self {
        use Direction::*;

        match value {
            Up => -1,
            Down => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_convert() {
        assert_eq!(
            zigzag_convert("PAYPALISHIRING", 3),
            "PAHNAPLSIIGYIR".to_owned()
        );

        assert_eq!(
            zigzag_convert("PAYPALISHIRING", 4),
            "PINALSIGYAHRPI".to_owned()
        );

        assert_eq!(
            zigzag_convert("PAYPALISHIRING", 1),
            "PAYPALISHIRING".to_owned()
        );
    }
}
