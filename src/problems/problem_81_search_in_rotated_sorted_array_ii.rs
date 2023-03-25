//! 81. Search in Rotated Sorted Array II
//!
//! Medium
//!
//! There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).
//! Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the
//! resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed).
//! For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].
//! Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.
//! You must decrease the overall operation steps as much as possible.
//!
//! Example 1:
//! Input: nums = [2,5,6,0,0,1,2], target = 0
//! Output: true
//!
//! Example 2:
//! Input: nums = [2,5,6,0,0,1,2], target = 3
//! Output: false
//!
//! Constraints:
//! 1 <= nums.length <= 5000
//! -104 <= nums[i] <= 104
//! nums is guaranteed to be rotated at some pivot.
//! -104 <= target <= 104
//!
//! Follow up: This problem is similar to Search in Rotated Sorted Array, but nums may contain duplicates. Would this affect the runtime complexity? How and why?

pub fn search_rotated_sorted(nums: &[i32], target: i32) -> Option<usize> {
    if nums.len() == 1 {
        return match nums[0] == target {
            true => Some(0),
            false => None,
        };
    }

    let rotation_idx = find_rotation_index(nums);
    if let Some(idx) = binary_search_in_range(nums, target, 0, rotation_idx - 1) {
        return Some(idx);
    }

    if let Some(idx) = binary_search_in_range(nums, target, rotation_idx, nums.len() - 1) {
        return Some(idx);
    }

    None
}

fn find_rotation_index(nums: &[i32]) -> usize {
    assert!(!nums.is_empty());

    let mut low = 0;
    let mut high = nums.len() - 1;

    while low < high - 1 && nums[low] == nums[low + 1] {
        low += 1;
    }

    while low + 1 < high && nums[high] == nums[high - 1] {
        high -= 1;
    }

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
        assert!(search_rotated_sorted(&[2, 5, 6, 0, 0, 1, 2], 0).is_some());
    }

    #[test]
    fn test_search_rotated_sorted_2() {
        assert!(search_rotated_sorted(&[2, 5, 6, 0, 0, 1, 2], 3).is_none());
    }

    #[test]
    fn test_search_rotated_sorted_3() {
        assert!(search_rotated_sorted(&[1], 0).is_none());
    }

    #[test]
    fn test_search_rotated_sorted_4() {
        assert!(search_rotated_sorted(&[1], 1).is_some());
    }

    #[test]
    fn test_search_rotated_sorted_5() {
        assert!(search_rotated_sorted(
            &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
            2
        )
        .is_some());
    }

    #[test]
    fn test_find_rotation_index() {
        assert_eq!(find_rotation_index(&[4, 5, 6, 0, 1, 2, 3]), 3);
        assert_eq!(find_rotation_index(&[4, 4, 4, 0, 1, 2, 3]), 3);
        assert_eq!(find_rotation_index(&[4, 4, 4, 0, 1, 1, 1]), 3);
        assert_eq!(find_rotation_index(&[4, 4, 4, 0, 0, 0, 0]), 3);
        assert_eq!(
            find_rotation_index(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1]),
            14
        );
    }
}
