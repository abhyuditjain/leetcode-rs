//! 2218. Maximum Value of K Coins From Piles
//!
//! Hard
//!
//! There are n piles of coins on a table. Each pile consists of a positive number of coins of assorted denominations.
//! In one move, you can choose any coin on top of any pile, remove it, and add it to your wallet.
//! Given a list piles, where piles[i] is a list of integers denoting the composition of the ith pile from top to bottom,
//! and a positive integer k, return the maximum total value of coins you can have in your wallet if you choose exactly k coins optimally.
//!
//! Example 1:
//! Input: piles = [[1,100,3],[7,8,9]], k = 2
//! Output: 101
//! Explanation:
//! The above diagram shows the different ways we can choose k coins.
//! The maximum total we can obtain is 101.
//!
//! Example 2:
//! Input: piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
//! Output: 706
//! Explanation:
//! The maximum total can be obtained if we choose all coins from the last pile.
//!
//! Constraints:
//! n == piles.length
//! 1 <= n <= 1000
//! 1 <= piles[i][j] <= 10^5
//! 1 <= k <= sum(piles[i].length) <= 2000

pub enum Algorithm {
    Memoization,
    Tabulation,
}

pub fn max_value_of_coins(piles: &[Vec<i32>], k: i32, alg: Algorithm) -> i32 {
    let piles = piles
        .iter()
        .map(|pile| pile.iter().map(|&coin| coin as usize).collect())
        .collect::<Vec<Vec<_>>>();

    match alg {
        Algorithm::Memoization => max_value_of_k_coins_memoization(&piles, k as usize) as i32,
        Algorithm::Tabulation => max_value_of_k_coins_tabulation(&piles, k as usize) as i32,
    }
}

fn max_value_of_k_coins_memoization(piles: &[Vec<usize>], k: usize) -> usize {
    fn recurse(piles: &[Vec<usize>], i: usize, k: usize, cache: &mut [Vec<usize>]) -> usize {
        // No more piles remaining or no more coins left to collect
        if i >= piles.len() || k == 0 {
            return 0;
        }

        if cache[i][k] > 0 {
            return cache[i][k];
        }

        // Let's not pick any coins from i'th pile and solve for remaining piles.
        let mut max_sum = recurse(piles, i + 1, k, cache);

        let mut picked_sum = 0;
        // start picking 1, 2, 3....min(k, piles[i].len()) coins from current pile
        let max_coins_we_can_take = k.min(piles[i].len());
        for j in 0..max_coins_we_can_take {
            let coin_value = piles[i][j];
            // The sum after picking (j+1) coins from current pile (j = 0 doesn't mean we picked 0 as j is 0 based)
            picked_sum += coin_value;

            // The sum after picking k - (j + 1) coins from rest of the piles
            let next_sum = recurse(piles, i + 1, k - j - 1, cache);

            max_sum = max_sum.max(picked_sum + next_sum);
        }

        cache[i][k] = max_sum;

        max_sum
    }

    let mut cache = vec![vec![0; k + 1]; piles.len() + 1];
    recurse(piles, 0, k, &mut cache)
}

fn max_value_of_k_coins_tabulation(piles: &[Vec<usize>], k: usize) -> usize {
    let mut dp = vec![vec![0; k + 1]; piles.len()];

    for i in 0..piles.len() {
        let pile_size = piles[i].len();
        for num_coins in 1..=k {
            let mut max_sum = match i.checked_sub(1) {
                Some(x) => dp[x][num_coins],
                None => 0,
            };

            let mut picked_sum = 0;
            let max_coins_we_can_take = num_coins.min(pile_size);

            //now consider picking some coins
            for j in 0..max_coins_we_can_take {
                let coin_value = piles[i][j];
                // The sum after picking (j+1) coins from current pile (j = 0 doesn't mean we picked 0 as j is 0 based)
                picked_sum += coin_value;

                let next_max_sum = match i.checked_sub(1) {
                    Some(x) => dp[x][num_coins - j - 1],
                    None => 0,
                };
                max_sum = max_sum.max(picked_sum + next_max_sum);
            }
            dp[i][num_coins] = max_sum;
        }
    }

    dp[piles.len() - 1][k]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_value_of_k_coins_memoization_1() {
        assert_eq!(
            max_value_of_k_coins_memoization(&[vec![1, 100, 3], vec![7, 8, 9]], 2),
            101
        );
    }

    #[test]
    fn test_max_value_of_k_coins_memoization_2() {
        assert_eq!(
            max_value_of_k_coins_memoization(
                &[
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );
    }

    #[test]
    fn test_max_value_of_k_coins_tabulation_1() {
        assert_eq!(
            max_value_of_k_coins_tabulation(&[vec![1, 100, 3], vec![7, 8, 9]], 2),
            101
        );
    }

    #[test]
    fn test_max_value_of_k_coins_tabulation_2() {
        assert_eq!(
            max_value_of_k_coins_tabulation(
                &[
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![100],
                    vec![1, 1, 1, 1, 1, 1, 700]
                ],
                7
            ),
            706
        );
    }
}
