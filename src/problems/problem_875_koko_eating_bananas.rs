//! 875. Koko Eating Bananas
//!
//! Medium
//!
//! Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.
//! Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile.
//! If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.
//! Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
//!
//! Return the minimum integer k such that she can eat all the bananas within h hours.
//!
//! Example 1:
//! Input: piles = [3,6,7,11], h = 8
//! Output: 4
//!
//! Example 2:
//! Input: piles = [30,11,23,4,20], h = 5
//! Output: 30
//!
//! Example 3:
//! Input: piles = [30,11,23,4,20], h = 6
//! Output: 23
//!
//! Constraints:
//! 1 <= piles.length <= 104
//! piles.length <= h <= 109
//! 1 <= piles[i] <= 109

/// This function returns the minimum rate which is required to eat all the bananas within `max_hours`
pub fn min_eating_speed(piles: &[usize], max_hours: usize) -> usize {
    assert!(!piles.is_empty());

    let mut low_rate = 1;
    let mut high_rate = *piles.iter().max().unwrap();

    while low_rate < high_rate {
        let mid_rate = low_rate + (high_rate - low_rate) / 2;

        if can_eat(piles, mid_rate, max_hours) {
            high_rate = mid_rate;
        } else {
            low_rate = mid_rate + 1;
        }
    }

    low_rate
}

fn can_eat(piles: &[usize], rate: usize, max_hours: usize) -> bool {
    let total_time = piles
        .iter()
        .map(|&pile| (pile + rate - 1) / rate)
        .sum::<usize>();

    total_time <= max_hours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_eat() {
        assert!(can_eat(&[1], 1, 1));
        assert!(can_eat(&[1, 1, 1, 1], 1, 4));
        assert!(!can_eat(&[1, 1, 1, 1], 1, 3));
    }

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(min_eating_speed(&[3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 5), 30);
        assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 6), 23);
        assert_eq!(min_eating_speed(&[1000000000], 2), 500000000);
    }
}
