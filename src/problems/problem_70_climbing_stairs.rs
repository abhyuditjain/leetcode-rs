//! 70. Climbing Stairs
//!
//! Easy
//!
//! You are climbing a staircase. It takes n steps to reach the top.
//! Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//!
//! Example 1:
//! Input: n = 2
//! Output: 2
//! Explanation: There are two ways to climb to the top.
//! 1. 1 step + 1 step
//! 2. 2 steps
//!
//! Example 2:
//! Input: n = 3
//! Output: 3
//! Explanation: There are three ways to climb to the top.
//! 1. 1 step + 1 step + 1 step
//! 2. 1 step + 2 steps
//! 3. 2 steps + 1 step
//!
//! Constraints:
//! 1 <= n <= 45

pub fn count_distinct_ways_to_climb_stairs(n: usize) -> usize {
    if n < 2 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 1..=n {
        let c = a + b;
        a = b;
        b = c;
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_distinct_ways_to_climb_stairs() {
        assert_eq!(count_distinct_ways_to_climb_stairs(1), 1);
        assert_eq!(count_distinct_ways_to_climb_stairs(2), 2);
        assert_eq!(count_distinct_ways_to_climb_stairs(3), 3);
        assert_eq!(count_distinct_ways_to_climb_stairs(4), 5);
        assert_eq!(count_distinct_ways_to_climb_stairs(5), 8);
    }
}
