//! 57. Insert Interval
//!
//! Medium
//!
//! You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi]
//! represent the start and the end of the ith interval and intervals is sorted in ascending order by starti.
//! You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
//! Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and
//! intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
//!
//! Return intervals after the insertion.
//!
//! Example 1:
//! Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
//! Output: [[1,5],[6,9]]
//!
//! Example 2:
//! Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
//! Output: [[1,2],[3,10],[12,16]]
//! Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//!
//! Constraints:
//! 0 <= intervals.length <= 10^4
//! intervals[i].length == 2
//! 0 <= starti <= endi <= 10^5
//! intervals is sorted by starti in ascending order.
//! newInterval.length == 2
//! 0 <= start <= end <= 10^5

use super::problem_56_merge_intervals::merge_intervals;

pub fn insert_interval(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut intervals = intervals
        .into_iter()
        .map(|x| (x[0] as usize, x[1] as usize))
        .collect::<Vec<_>>();

    intervals.push((new_interval[0] as usize, new_interval[1] as usize));

    let merged = merge_intervals(&mut intervals);

    merged
        .iter()
        .map(|&(start, end)| vec![start as i32, end as i32])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_interval_1() {
        assert_eq!(
            insert_interval(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn test_insert_interval_2() {
        assert_eq!(
            insert_interval(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
