//! 4. Median of Two Sorted Arrays
//!
//! Hard
//!
//! Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
//!
//! The overall run time complexity should be O(log (m+n)).
//!
//! Example 1:
//! Input: nums1 = [1,3], nums2 = [2]
//! Output: 2.00000
//! Explanation: merged array = [1,2,3] and median is 2.
//!
//! Example 2:
//! Input: nums1 = [1,2], nums2 = [3,4]
//! Output: 2.50000
//! Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//!
//! Constraints:
//! nums1.length == m
//! nums2.length == n
//! 0 <= m <= 1000
//! 0 <= n <= 1000
//! 1 <= m + n <= 2000
//! -106 <= nums1[i], nums2[i] <= 106

pub fn median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    if nums1.len() > nums2.len() {
        return median_sorted_arrays(nums2, nums1);
    }

    let n1 = nums1.len();
    let n2 = nums2.len();

    let mut low = 0;
    let mut high = n1;

    while low <= high {
        let cut1 = low + (high - low) / 2;
        let cut2 = (n1 + n2 + 1) / 2 - cut1;

        let (left1, right1) = get_left_right(cut1, nums1);
        let (left2, right2) = get_left_right(cut2, nums2);

        if left1 <= right2 && left2 <= right1 {
            if (n1 + n2) % 2 == 0 {
                return (left1.max(left2) + right1.min(right2)) as f64 / 2_f64;
            }
            return left1.max(left2) as f64;
        }

        if left1 > right2 {
            high = cut1 - 1;
        } else {
            low = cut1 + 1;
        }
    }

    0_f64
}

fn get_left_right(cut: usize, nums: &[i32]) -> (i32, i32) {
    let left = if cut == 0 { i32::MIN } else { nums[cut - 1] };
    let right = if cut == nums.len() {
        i32::MAX
    } else {
        nums[cut]
    };

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_sorted_arrays() {
        assert_eq!(median_sorted_arrays(&[1, 3], &[2]), 2_f64);
        assert_eq!(median_sorted_arrays(&[1, 2], &[3, 4]), 2.5_f64);
        assert_eq!(median_sorted_arrays(&[], &[1]), 1_f64);
    }
}
