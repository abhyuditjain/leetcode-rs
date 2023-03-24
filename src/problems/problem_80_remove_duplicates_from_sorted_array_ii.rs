//! 80. Remove Duplicates from Sorted Array II
//!
//! Medium
//!
//! Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most
//! twice.
//! The relative order of the elements should be kept the same.
//! Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of
//! the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the
//! final result. It does not matter what you leave beyond the first k elements.
//! Return k after placing the final result in the first k slots of nums.
//! Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
//!
//! Example 1:
//! Input: nums = [1,1,1,2,2,3]
//! Output: 5, nums = [1,1,2,2,3,_]
//! Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
//! It does not matter what you leave beyond the returned k (hence they are underscores).
//!
//! Example 2:
//! Input: nums = [0,0,1,1,1,1,2,3,3]
//! Output: 7, nums = [0,0,1,1,2,3,3,_,_]
//! Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
//! It does not matter what you leave beyond the returned k (hence they are underscores).
//!
//! Constraints:
//! 1 <= nums.length <= 3 * 10^4
//! -10^4 <= nums[i] <= 10^4
//! nums is sorted in non-decreasing order.

pub fn remove_duplicates_from_arr(nums: &mut [i32]) -> usize {
    assert!(!nums.is_empty());

    let mut count = 1;
    let mut j = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count += 1;
        } else {
            count = 1;
        }

        if count <= 2 {
            nums[j] = nums[i];
            j += 1;
        }
    }

    j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_from_arr_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = remove_duplicates_from_arr(&mut nums);

        assert_eq!(&nums[..k], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_remove_duplicates_from_arr_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k = remove_duplicates_from_arr(&mut nums);

        assert_eq!(&nums[..k], &[0, 0, 1, 1, 2, 3, 3]);
    }
}
