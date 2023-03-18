//! 35. Search Insert Position
//!
//! Easy
//!
//! Given a sorted array of distinct integers and a target value, return the index if the target is found.
//! If not, return the index where it would be if it were inserted in order.
//! You must write an algorithm with O(log n) runtime complexity.
//!
//! Example 1:
//! Input: nums = [1,3,5,6], target = 5
//! Output: 2
//!
//! Example 2:
//! Input: nums = [1,3,5,6], target = 2
//! Output: 1
//!
//! Example 3:
//! Input: nums = [1,3,5,6], target = 7
//! Output: 4
//!
//! Constraints:
//! 1 <= nums.length <= 104
//! -104 <= nums[i] <= 104
//! nums contains distinct values sorted in ascending order.
//! -104 <= target <= 104

pub fn search_insert_position(nums: &[i32], target: i32) -> usize {
    let mut low = 0_i32;
    let mut high = nums.len() as i32 - 1;

    while low <= high {
        let mid = (low + (high - low) / 2) as usize;

        if nums[mid] == target {
            return mid;
        }

        if nums[mid] > target {
            high = mid as i32 - 1;
        } else {
            low = mid as i32 + 1;
        }
    }

    low as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert_position_1() {
        assert_eq!(search_insert_position(&[1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test_search_insert_position_2() {
        assert_eq!(search_insert_position(&[1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_search_insert_position_3() {
        assert_eq!(search_insert_position(&[1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_search_insert_position_4() {
        assert_eq!(search_insert_position(&[1, 3, 5, 6], 0), 0);
    }
}
