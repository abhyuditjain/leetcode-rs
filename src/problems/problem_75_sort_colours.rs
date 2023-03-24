//! 75. Sort Colors
//!
//! Medium
//!
//! Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
//! We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
//! You must solve this problem without using the library's sort function.
//!
//! Example 1:
//! Input: nums = [2,0,2,1,1,0]
//! Output: [0,0,1,1,2,2]
//!
//! Example 2:
//! Input: nums = [2,0,1]
//! Output: [0,1,2]
//!
//! Constraints:
//! n == nums.length
//! 1 <= n <= 300
//! nums[i] is either 0, 1, or 2.
//!
//! Follow up: Could you come up with a one-pass algorithm using only constant extra space?

use std::cmp::Ordering;

pub enum Algorithm {
    LinearTimeLinearSpace,
    LinearTimeConstantSpace,
}

pub fn sort_colours(nums: &mut [i32], alg: Algorithm) {
    match alg {
        Algorithm::LinearTimeLinearSpace => count_sort(nums),
        Algorithm::LinearTimeConstantSpace => dutch_national_flag(nums),
    }
}

fn dutch_national_flag(nums: &mut [i32]) {
    let mut low = 0;
    let mut high = nums.len() - 1;

    let mut i = 0;

    while i <= high {
        match nums[i].cmp(&1) {
            Ordering::Less => {
                nums.swap(low, i);
                low += 1;
                i += 1;
            }
            Ordering::Equal => i += 1,
            Ordering::Greater => {
                nums.swap(i, high);

                high = match high.checked_sub(1) {
                    Some(high) => high,
                    None => break,
                }
            }
        }
    }
}

fn count_sort(nums: &mut [i32]) {
    let mut counts = [0; 3];

    for n in nums.iter() {
        counts[*n as usize] += 1;
    }

    let mut k = 0;

    for v in nums {
        while counts[k] == 0 {
            k += 1;
        }

        *v = k as i32;
        counts[k] -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_sort() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let correct = vec![0, 0, 1, 1, 2, 2];
        count_sort(&mut nums);
        assert_eq!(nums, correct);

        let mut nums = vec![2, 0, 1];
        let correct = vec![0, 1, 2];
        count_sort(&mut nums);
        assert_eq!(nums, correct);

        let mut nums = vec![0, 0, 0];
        let correct = vec![0, 0, 0];
        count_sort(&mut nums);
        assert_eq!(nums, correct);

        let mut nums = vec![2, 2, 2];
        let correct = vec![2, 2, 2];
        count_sort(&mut nums);
        assert_eq!(nums, correct);
    }

    #[test]
    fn test_dutch_national_flag() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let correct = vec![0, 0, 1, 1, 2, 2];
        dutch_national_flag(&mut nums);
        assert_eq!(nums, correct);

        let mut nums = vec![2, 0, 1];
        let correct = vec![0, 1, 2];
        dutch_national_flag(&mut nums);
        assert_eq!(nums, correct);

        let mut nums = vec![0, 0, 0];
        let correct = vec![0, 0, 0];
        dutch_national_flag(&mut nums);
        assert_eq!(nums, correct);

        let mut nums = vec![2, 2, 2];
        let correct = vec![2, 2, 2];
        dutch_national_flag(&mut nums);
        assert_eq!(nums, correct);
    }
}
