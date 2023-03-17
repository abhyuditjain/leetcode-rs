//! 34. Find First and Last Position of Element in Sorted Array
//!
//! Medium
//!
//! Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
//! If target is not found in the array, return [-1, -1].
//!
//! You must write an algorithm with O(log n) runtime complexity.
//!
//! Example 1:
//! Input: nums = [5,7,7,8,8,10], target = 8
//! Output: [3,4]
//!
//! Example 2:
//! Input: nums = [5,7,7,8,8,10], target = 6
//! Output: [-1,-1]
//!
//! Example 3:
//! Input: nums = [], target = 0
//! Output: [-1,-1]
//!
//! Constraints:
//! 0 <= nums.length <= 105
//! -109 <= nums[i] <= 109
//! nums is a non-decreasing array.
//! -109 <= target <= 109

use std::cmp::Ordering;

pub fn find_range(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    if let Some(first) = find_position(nums, target, true) {
        let last = find_position(nums, target, false).unwrap();
        Some((first, last))
    } else {
        None
    }
}

fn find_position(nums: &[i32], target: i32, search_first: bool) -> Option<usize> {
    let mut low = 0_i32;
    let mut high = nums.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        match target.cmp(&nums[mid as usize]) {
            Ordering::Less => high = mid - 1,
            Ordering::Greater => low = mid + 1,
            Ordering::Equal => {
                if search_first {
                    if mid == low || nums[mid as usize] != nums[mid as usize - 1] {
                        return Some(mid as usize);
                    }
                    high = mid - 1;
                } else {
                    if mid == high || nums[mid as usize] != nums[mid as usize + 1] {
                        return Some(mid as usize);
                    }
                    low = mid + 1;
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_range_1() {
        assert_eq!(find_range(&[5, 7, 7, 8, 8, 10], 8), Some((3, 4)));
    }

    #[test]
    fn test_find_range_2() {
        assert_eq!(find_range(&[5, 7, 7, 8, 8, 10], 6), None);
    }

    #[test]
    fn test_find_range_3() {
        assert_eq!(find_range(&[1], 1), Some((0, 0)));
    }

    #[test]
    fn test_find_range_4() {
        assert_eq!(find_range(&[], 8), None);
    }

    #[test]
    fn test_find_position() {
        assert_eq!(find_position(&[1, 1, 2, 2, 3, 3], 2, true), Some(2));
        assert_eq!(find_position(&[1, 1, 2, 2, 3, 3], 2, false), Some(3));

        assert_eq!(find_position(&[1, 1, 2, 2, 3, 3], 1, true), Some(0));
        assert_eq!(find_position(&[1, 1, 2, 2, 3, 3], 1, false), Some(1));

        assert_eq!(find_position(&[1, 1, 2, 2, 3, 3], 3, true), Some(4));
        assert_eq!(find_position(&[1, 1, 2, 2, 3, 3], 3, false), Some(5));
    }
}
