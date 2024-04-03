//! 31. Next Permutation
//!
//! Medium
//!
//! A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
//! For example, for arr = [1,2,3], the following are all the permutations of arr:
//! [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
//!
//! The next permutation of an array of integers is the next lexicographically greater permutation of its integer.
//! More formally, if all the permutations of the array are sorted in one container according to their lexicographical order,
//! then the next permutation of that array is the permutation that follows it in the sorted container.
//! If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).
//!
//! For example, the next permutation of arr = [1,2,3] is [1,3,2].
//! Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
//! While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.
//! Given an array of integers nums, find the next permutation of nums.
//! The replacement must be in place and use only constant extra memory.
//!
//! Example 1:
//! Input: nums = [1,2,3]
//! Output: [1,3,2]
//!
//! Example 2:
//! Input: nums = [3,2,1]
//! Output: [1,2,3]
//!
//! Example 3:
//! Input: nums = [1,1,5]
//! Output: [1,5,1]
//!
//! Constraints:
//! 1 <= nums.length <= 100
//! 0 <= nums[i] <= 100

pub fn make_next_permutation(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }

    match get_index_of_decreasing_pair_from_end(nums) {
        Some(start) => {
            let end = get_index_of_next_greater_element_from_end_than_element_at(nums, start - 1)
                .unwrap();
            nums.swap(start - 1, end);

            reverse_from(nums, start)
        }
        None => reverse_from(nums, 0),
    }
}

fn get_index_of_decreasing_pair_from_end(nums: &[i32]) -> Option<usize> {
    if nums.len() < 2 {
        return None;
    }
    (1..nums.len()).rev().find(|&i| nums[i - 1] < nums[i])
}

fn get_index_of_next_greater_element_from_end_than_element_at(
    nums: &[i32],
    start: usize,
) -> Option<usize> {
    (start..nums.len()).rev().find(|&i| nums[i] > nums[start])
}

fn reverse_from(nums: &mut [i32], start: usize) {
    let mut i = start;
    let mut j = nums.len() - 1;

    while i < j {
        nums.swap(i, j);
        i += 1;
        j -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_next_permutation_1() {
        let mut nums = vec![1, 2, 3];
        make_next_permutation(&mut nums);

        assert_eq!(nums, &[1, 3, 2]);
    }

    #[test]
    fn test_make_next_permutation_2() {
        let mut nums = vec![3, 2, 1];
        make_next_permutation(&mut nums);

        assert_eq!(nums, &[1, 2, 3]);
    }

    #[test]
    fn test_make_next_permutation_3() {
        let mut nums = vec![1, 1, 5];
        make_next_permutation(&mut nums);

        assert_eq!(nums, &[1, 5, 1]);
    }

    #[test]
    fn test_get_index_of_decreasing_pair_from_end_1() {
        assert_eq!(get_index_of_decreasing_pair_from_end(&[1, 2, 3]), Some(2));
        assert_eq!(get_index_of_decreasing_pair_from_end(&[3, 2, 1]), None);
        assert_eq!(get_index_of_decreasing_pair_from_end(&[1, 1, 5]), Some(2));
    }

    #[test]
    fn test_get_index_of_next_greater_element_from_end_than_element_at_1() {
        assert_eq!(
            get_index_of_next_greater_element_from_end_than_element_at(&[1, 2, 3], 0),
            Some(2)
        );

        assert_eq!(
            get_index_of_next_greater_element_from_end_than_element_at(&[1, 2, 3], 1),
            Some(2)
        );

        assert_eq!(
            get_index_of_next_greater_element_from_end_than_element_at(&[3, 2, 1], 0),
            None,
        );
        assert_eq!(
            get_index_of_next_greater_element_from_end_than_element_at(&[1, 1, 5], 0),
            Some(2)
        );
    }

    #[test]
    fn test_reverse_from() {
        let mut nums = vec![3, 2, 1, 1, 2, 3];
        reverse_from(&mut nums, 3);
        assert_eq!(nums, &[3, 2, 1, 3, 2, 1]);

        reverse_from(&mut nums, 0);
        assert_eq!(nums, &[1, 2, 3, 1, 2, 3]);
    }
}
