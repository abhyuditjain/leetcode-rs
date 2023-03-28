//! 96. Unique Binary Search Trees
//!
//! Medium
//!
//! Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
//!
//! Example 1:
//! Input: n = 3
//! Output: 5
//!
//! Example 2:
//! Input: n = 1
//! Output: 1
//!
//! Constraints:
//! 1 <= n <= 19

use std::collections::HashMap;

pub fn get_num_trees(n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut cache = HashMap::new();

    get_num_trees_helper(1, n, &mut cache)
}

fn get_num_trees_helper(
    start: usize,
    end: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if start >= end {
        return 1;
    }

    if let Some(count) = cache.get(&(start, end)) {
        return *count;
    }

    let mut count = 0;

    for i in start..=end {
        let left_count = get_num_trees_helper(start, i - 1, cache);
        let right_count = get_num_trees_helper(i + 1, end, cache);

        count += left_count * right_count;
    }

    cache.insert((start, end), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_trees_1() {
        assert_eq!(get_num_trees(3), 5);
    }

    #[test]
    fn test_get_num_trees_2() {
        assert_eq!(get_num_trees(1), 1);
    }
}
