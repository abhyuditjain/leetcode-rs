//! 22. Generate Parentheses
//!
//! Medium
//!
//! Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//!
//! Example 1:
//! Input: n = 3
//! Output: ["((()))","(()())","(())()","()(())","()()()"]
//!
//! Example 2:
//! Input: n = 1
//! Output: ["()"]
//!
//! Constraints:
//! 1 <= n <= 8

pub fn generate_parenthesis(n: u8) -> Vec<String> {
    let mut combinations = vec![];
    let mut combination = "".to_string();
    backtrack(n, &mut combinations, &mut combination, 0, 0);
    combinations
}

fn backtrack(
    max: u8,
    combinations: &mut Vec<String>,
    combination: &mut String,
    open_count: u8,
    close_count: u8,
) {
    if open_count == max && close_count == open_count {
        combinations.push(combination.clone());
        return;
    }

    if open_count < max {
        combination.push('(');
        backtrack(max, combinations, combination, open_count + 1, close_count);
        combination.pop();
    }

    if close_count < open_count {
        combination.push(')');
        backtrack(max, combinations, combination, open_count, close_count + 1);
        combination.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parentheses() {
        assert_eq!(
            generate_parenthesis(3),
            vec![
                "((()))".to_owned(),
                "(()())".to_owned(),
                "(())()".to_owned(),
                "()(())".to_owned(),
                "()()()".to_owned(),
            ]
        );

        assert_eq!(generate_parenthesis(1), vec!["()".to_owned()]);
    }
}
