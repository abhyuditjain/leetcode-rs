//! 879. Profitable Schemes
//!
//! Hard
//!
//! There is a group of n members, and a list of various crimes they could commit. The ith crime generates a profit[i] and requires group[i] members to participate in it. If a member participates in one crime, that member can't participate in another crime.
//! Let's call a profitable scheme any subset of these crimes that generates at least minProfit profit, and the total number of members participating in that subset of crimes is at most n.
//! Return the number of schemes that can be chosen. Since the answer may be very large, return it modulo 109 + 7.
//!
//! Example 1:
//! Input: n = 5, minProfit = 3, group = [2,2], profit = [2,3]
//! Output: 2
//! Explanation: To make a profit of at least 3, the group could either commit crimes 0 and 1, or just crime 1.
//! In total, there are 2 schemes.
//!
//! Example 2:
//! Input: n = 10, minProfit = 5, group = [2,3,5], profit = [6,7,8]
//! Output: 7
//! Explanation: To make a profit of at least 5, the group could commit any crimes, as long as they commit one.
//! There are 7 possible schemes: (0), (1), (2), (0,1), (0,2), (1,2), and (0,1,2).
//!
//! Constraints:
//! 1 <= n <= 100
//! 0 <= minProfit <= 100
//! 1 <= group.length <= 100
//! 1 <= group[i] <= 100
//! profit.length == group.length
//! 0 <= profit[i] <= 100

use std::collections::HashMap;

const MOD: i32 = 1000000007;

pub enum Algorithm {
    Memoization,
    Tabulation,
}

pub fn profitable_schemes(
    n: i32,
    min_profit: i32,
    group: Vec<i32>,
    profit: Vec<i32>,
    alg: Algorithm,
) -> i32 {
    match alg {
        Algorithm::Memoization => {
            profitable_schemes_memoization(n as usize, min_profit, &group, &profit)
        }
        Algorithm::Tabulation => {
            profitable_schemes_tabulation(n as usize, min_profit, &group, &profit)
        }
    }
}

fn profitable_schemes_memoization(n: usize, min_profit: i32, group: &[i32], profit: &[i32]) -> i32 {
    fn memoize(
        state: (usize, i32, i32),
        n: usize,
        min_profit: i32,
        group: &[i32],
        profits: &[i32],
        cache: &mut HashMap<(usize, i32, i32), i32>,
    ) -> i32 {
        let (idx, count, profit) = state;
        if idx == group.len() {
            if profit >= min_profit {
                return 1;
            }
            return 0;
        }

        if let Some(&total_ways) = cache.get(&state) {
            return total_ways;
        }

        let mut total_ways = memoize(
            (idx + 1, count, profit),
            n,
            min_profit,
            group,
            profits,
            cache,
        );
        if count + group[idx] <= n as i32 {
            total_ways += memoize(
                (
                    idx + 1,
                    count + group[idx],
                    min_profit.min(profit + profits[idx]),
                ),
                n,
                min_profit,
                group,
                profits,
                cache,
            );
        }

        total_ways %= MOD;

        cache.insert(state, total_ways);

        total_ways
    }

    let mut cache = HashMap::new();

    memoize((0, 0, 0), n, min_profit, group, profit, &mut cache)
}

fn profitable_schemes_tabulation(n: usize, min_profit: i32, group: &[i32], profits: &[i32]) -> i32 {
    let mut dp = vec![vec![vec![0; 101]; 101]; 101];

    for count in 0..=n {
        dp[group.len()][count][min_profit as usize] = 1;
    }

    for idx in (0..group.len()).rev() {
        for count in 0..=n {
            for profit in 0..=(min_profit as usize) {
                dp[idx][count][profit] = dp[idx + 1][count][profit];
                if count + group[idx] as usize <= n {
                    // Adding ways to get profitable schemes, including this crime.
                    dp[idx][count][profit] = (dp[idx][count][profit]
                        + dp[idx + 1][count + group[idx] as usize]
                            [min_profit.min(profit as i32 + profits[idx]) as usize])
                        % MOD;
                }
            }
        }
    }

    dp[0][0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profitable_schemes_memoization() {
        assert_eq!(profitable_schemes_memoization(5, 3, &[2, 2], &[2, 3]), 2);
        assert_eq!(
            profitable_schemes_memoization(10, 5, &[2, 3, 5], &[6, 7, 8]),
            7
        );
    }

    #[test]
    fn test_profitable_schemes_tabulation() {
        assert_eq!(profitable_schemes_tabulation(5, 3, &[2, 2], &[2, 3]), 2);
        assert_eq!(
            profitable_schemes_tabulation(10, 5, &[2, 3, 5], &[6, 7, 8]),
            7
        );
    }
}
