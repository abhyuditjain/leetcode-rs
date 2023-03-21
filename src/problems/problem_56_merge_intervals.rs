//! 56. Merge Intervals
//!
//! Medium
//!
//! Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals,
//! and return an array of the non-overlapping intervals that cover all the intervals in the input.
//!
//! Example 1:
//! Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
//! Output: [[1,6],[8,10],[15,18]]
//! Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
//!
//! Example 2:
//! Input: intervals = [[1,4],[4,5]]
//! Output: [[1,5]]
//! Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//!
//! Constraints:
//! 1 <= intervals.length <= 10^4
//! intervals[i].length == 2
//! 0 <= starti <= endi <= 10^4
//!

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals
        .into_iter()
        .map(|x| (x[0] as usize, x[1] as usize))
        .collect::<Vec<_>>();
    let merged = merge_intervals(&mut intervals);

    merged
        .iter()
        .map(|&(start, end)| vec![start as i32, end as i32])
        .collect()
}

pub fn merge_intervals(intervals: &mut [(usize, usize)]) -> Vec<(usize, usize)> {
    intervals.sort_by_key(|(start, _end)| *start);
    let mut merged = vec![];

    for &(start, end) in intervals.iter() {
        match merged.last_mut() {
            None => merged.push((start, end)),
            Some((_last_start, last_end)) => {
                if *last_end < start {
                    merged.push((start, end));
                } else {
                    *last_end = (*last_end).max(end);
                }
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        assert_eq!(
            merge_intervals(&mut [(1, 3), (2, 6), (8, 10), (15, 18)]),
            vec![(1, 6), (8, 10), (15, 18)]
        );
    }

    #[test]
    fn test_merge_intervals_2() {
        assert_eq!(merge_intervals(&mut [(1, 4), (4, 5)]), vec![(1, 5)]);
    }
}
