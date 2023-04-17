//! 985. Sum of Even Numbers After Queries
//!
//! Medium
//!
//! You are given an integer array nums and an array queries where queries[i] = [vali, indexi].
//! For each query i, first, apply nums[indexi] = nums[indexi] + vali, then print the sum of the even values of nums.
//! Return an integer array answer where answer[i] is the answer to the ith query.
//!
//! Example 1:
//! Input: nums = [1,2,3,4], queries = [[1,0],[-3,1],[-4,0],[2,3]]
//! Output: [8,6,2,4]
//! Explanation: At the beginning, the array is [1,2,3,4].
//! After adding 1 to nums[0], the array is [2,2,3,4], and the sum of even values is 2 + 2 + 4 = 8.
//! After adding -3 to nums[1], the array is [2,-1,3,4], and the sum of even values is 2 + 4 = 6.
//! After adding -4 to nums[0], the array is [-2,-1,3,4], and the sum of even values is -2 + 4 = 2.
//! After adding 2 to nums[3], the array is [-2,-1,3,6], and the sum of even values is -2 + 6 = 4.
//!
//! Example 2:
//! Input: nums = [1], queries = [[4,0]]
//! Output: [0]
//!
//! Constraints:
//! 1 <= nums.length <= 10^4
//! -10^4 <= nums[i] <= 10^4
//! 1 <= queries.length <= 10^4
//! -10^4 <= vali <= 10^4
//! 0 <= indexi < nums.length

pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let queries = queries
        .into_iter()
        .map(|query| (query[0], query[1] as usize))
        .collect::<Vec<_>>();

    sum_even_nums_after_queries(&mut nums, &queries)
}

fn sum_even_nums_after_queries(nums: &mut [i32], queries: &[(i32, usize)]) -> Vec<i32> {
    let mut sum_even = nums.iter().filter(|&&x| x % 2 == 0).sum();

    queries
        .iter()
        .map(|&(val, i)| {
            if nums[i] % 2 == 0 {
                sum_even -= nums[i];
            }

            nums[i] += val;

            if nums[i] % 2 == 0 {
                sum_even += nums[i];
            }

            sum_even
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_nums_after_queries_1() {
        assert_eq!(
            sum_even_nums_after_queries(&mut [1, 2, 3, 4], &[(1, 0), (-3, 1), (-4, 0), (2, 3)]),
            vec![8, 6, 2, 4]
        );
    }

    #[test]
    fn test_sum_even_nums_after_queries_2() {
        assert_eq!(sum_even_nums_after_queries(&mut [1], &[(4, 0)]), vec![0]);
    }
}
