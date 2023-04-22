//! 518. Coin Change II
//!
//! Medium
//!
//! You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
//! Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.
//! You may assume that you have an infinite number of each kind of coin.
//! The answer is guaranteed to fit into a signed 32-bit integer.
//!
//! Example 1:
//! Input: amount = 5, coins = [1,2,5]
//! Output: 4
//! Explanation: there are four ways to make up the amount:
//! 5=5
//! 5=2+2+1
//! 5=2+1+1+1
//! 5=1+1+1+1+1
//!
//! Example 2:
//! Input: amount = 3, coins = [2]
//! Output: 0
//! Explanation: the amount of 3 cannot be made up just with coins of 2.
//!
//! Example 3:
//! Input: amount = 10, coins = [10]
//! Output: 1
//!
//! Constraints:
//! 1 <= coins.length <= 300
//! 1 <= coins[i] <= 5000
//! All the values of coins are unique.
//! 0 <= amount <= 5000

use std::collections::HashMap;

pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
    fn backtrack(
        coins: &[i32],
        idx: usize,
        amount_remaining: i32,
        cache: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if amount_remaining < 0 {
            return 0;
        }

        if let Some(&ways) = cache.get(&(idx, amount_remaining)) {
            return ways;
        }

        if amount_remaining == 0 {
            return 1;
        }

        let mut ways = 0;
        for i in idx..coins.len() {
            ways += backtrack(coins, i, amount_remaining - coins[i], cache);
        }

        cache.insert((idx, amount_remaining), ways);

        ways
    }

    coins.sort_unstable();
    let mut cache = HashMap::new();

    backtrack(&coins, 0, amount, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(change(5, vec![1, 2, 5]), 4);
        assert_eq!(change(3, vec![2]), 0);
        assert_eq!(change(10, vec![10]), 1);
    }
}
