//! 33. Search in Rotated Sorted Array
//!
//! Medium
//!
//! There is an integer array nums sorted in ascending order (with distinct values).
//! Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length)
//! such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed).
//! For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
//! Given the array nums after the possible rotation and an integer target,
//! return the index of target if it is in nums, or -1 if it is not in nums.
//! You must write an algorithm with O(log n) runtime complexity.
//!
//! Example 1:
//! Input: nums = [4,5,6,7,0,1,2], target = 0
//! Output: 4
//!
//! Example 2:
//! Input: nums = [4,5,6,7,0,1,2], target = 3
//! Output: -1
//!
//! Example 3:
//! Input: nums = [1], target = 0
//! Output: -1
//!
//! Constraints:
//! 1 <= nums.length <= 5000
//! -104 <= nums[i] <= 104
//! All values of nums are unique.
//! nums is an ascending array that is possibly rotated.
//! -104 <= target <= 104

pub fn search_rotated_sorted(nums: &[i32], target: i32) -> Option<usize> {
    if nums.len() == 1 {
        if nums[0] == target {
            return Some(0);
        } else {
            return None;
        }
    }

    let rotation_idx = find_rotation_index(nums);
    if let Some(i) = binary_search_in_range(nums, target, 0, rotation_idx - 1) {
        return Some(i);
    }

    if let Some(i) = binary_search_in_range(nums, target, rotation_idx, nums.len() - 1) {
        return Some(i);
    }

    None
}

fn find_rotation_index(nums: &[i32]) -> usize {
    assert!(!nums.is_empty());

    let mut low = 0;
    let mut high = nums.len() - 1;

    while low < high {
        let mid = low + (high - low) / 2;

        if nums[mid] > nums[high] {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

fn binary_search_in_range(nums: &[i32], target: i32, low: usize, high: usize) -> Option<usize> {
    let mut low = low as i32;
    let mut high = high as i32;
    while low <= high {
        let mid = low + (high - low) / 2;

        if nums[mid as usize] == target {
            return Some(mid as usize);
        }

        if nums[mid as usize] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated_sorted_1() {
        assert_eq!(search_rotated_sorted(&[4, 5, 6, 7, 0, 1, 2], 0), Some(4));
    }

    #[test]
    fn test_search_rotated_sorted_2() {
        assert_eq!(search_rotated_sorted(&[4, 5, 6, 7, 0, 1, 2], 3), None);
    }

    #[test]
    fn test_search_rotated_sorted_3() {
        assert_eq!(search_rotated_sorted(&[1], 0), None);
    }

    #[test]
    fn test_find_rotation_index() {
        assert_eq!(find_rotation_index(&[4, 5, 6, 7, 0, 1, 2]), 4);
        assert_eq!(find_rotation_index(&[4, 5, 6, 7]), 0);
        assert_eq!(find_rotation_index(&[3, 2, 1, 0]), 3);
        assert_eq!(find_rotation_index(&[1]), 0);
    }

    #[test]
    fn test_binary_search_in_range() {
        assert_eq!(binary_search_in_range(&[1, 2, 3, 4, 5], 3, 0, 4), Some(2));
        assert_eq!(binary_search_in_range(&[1, 2, 3, 4, 5], 3, 0, 3), Some(2));
        assert_eq!(binary_search_in_range(&[1, 2, 3, 4, 5], 3, 0, 2), Some(2));
        assert_eq!(binary_search_in_range(&[1, 2, 3, 4, 5], 3, 2, 2), Some(2));
        assert_eq!(binary_search_in_range(&[1, 2, 3, 4, 5], 3, 3, 2), None);
        assert_eq!(binary_search_in_range(&[1, 2, 3, 4, 5], 3, 0, 1), None);
    }
}
