//! 1235. Maximum Profit in Job Scheduling
//!
//! Hard
//!
//! We have n jobs, where every job is scheduled to be done from startTime[i] to endTime[i], obtaining a profit of profit[i].
//! You're given the startTime, endTime and profit arrays, return the maximum profit you can take such that there are no two jobs in the subset with overlapping time range.
//! If you choose a job that ends at time X you will be able to start another job that starts at time X.
//!
//! Example 1:
//! Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
//! Output: 120
//! Explanation: The subset chosen is the first and fourth job.
//! Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
//!
//! Example 2:
//! Input: startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,70,60]
//! Output: 150
//! Explanation: The subset chosen is the first, fourth and fifth job.
//! Profit obtained 150 = 20 + 70 + 60.
//!
//! Example 3:
//! Input: startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
//! Output: 6
//!
//! Constraints:
//! 1 <= startTime.length == endTime.length == profit.length <= 5 * 10^4
//! 1 <= startTime[i] < endTime[i] <= 10^9
//! 1 <= profit[i] <= 104

pub enum Algorithm {
    Memoization,
    Tabulation,
}

#[derive(PartialEq, Eq)]
struct Job {
    start_time: usize,
    end_time: usize,
    profit: usize,
}

pub fn job_scheduling(
    start_time: Vec<i32>,
    end_time: Vec<i32>,
    profit: Vec<i32>,
    alg: Algorithm,
) -> i32 {
    let jobs = get_sorted_jobs(start_time, end_time, profit);

    match alg {
        Algorithm::Memoization => max_profit_memoization(&jobs) as i32,
        Algorithm::Tabulation => max_profit_tabulation(&jobs) as i32,
    }
}

fn max_profit_memoization(jobs: &[Job]) -> usize {
    let mut cache = vec![None; jobs.len()];

    max_profit_memoization_helper(jobs, 0, &mut cache)
}

fn max_profit_memoization_helper(
    jobs: &[Job],
    start_idx: usize,
    cache: &mut [Option<usize>],
) -> usize {
    if start_idx == jobs.len() {
        return 0;
    }

    if let Some(profit) = cache[start_idx] {
        return profit;
    }

    let curr_job = &jobs[start_idx];
    let next_idx = find_next_job_index(jobs, curr_job.end_time);
    let profit_including_current_job =
        curr_job.profit + max_profit_memoization_helper(jobs, next_idx, cache);
    let profit_excluding_current_job = max_profit_memoization_helper(jobs, start_idx + 1, cache);

    let max_profit = profit_including_current_job.max(profit_excluding_current_job);

    cache[start_idx] = Some(max_profit);

    max_profit
}

fn max_profit_tabulation(jobs: &[Job]) -> usize {
    let mut dp = vec![0; jobs.len()];

    for pos in (0..jobs.len()).rev() {
        let curr_job = &jobs[pos];
        let next_idx = find_next_job_index(jobs, curr_job.end_time);

        let curr_profit = if next_idx == jobs.len() {
            curr_job.profit
        } else {
            curr_job.profit + dp[next_idx]
        };

        if pos == jobs.len() - 1 {
            dp[pos] = curr_profit;
        } else {
            dp[pos] = dp[pos + 1].max(curr_profit);
        }
    }

    dp[0]
}

fn find_next_job_index(jobs: &[Job], end_time: usize) -> usize {
    let (mut low, mut high, mut next_idx) = (0, jobs.len() - 1, jobs.len());

    while low <= high {
        let mid = low + (high - low) / 2;

        if jobs[mid].start_time >= end_time {
            next_idx = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    next_idx
}

fn get_sorted_jobs(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> Vec<Job> {
    let mut jobs = start_time
        .into_iter()
        .zip(end_time.into_iter())
        .zip(profit.into_iter())
        .map(|((start_time, end_time), profit)| {
            (start_time as usize, end_time as usize, profit as usize)
        })
        .map(|(start_time, end_time, profit)| Job {
            start_time,
            end_time,
            profit,
        })
        .collect::<Vec<_>>();

    jobs.sort_unstable_by_key(|x| x.start_time);

    jobs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_memoization_1() {
        let jobs = get_sorted_jobs(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]);

        assert_eq!(max_profit_memoization(&jobs), 120);
    }

    #[test]
    fn test_max_profit_memoization_2() {
        let jobs = get_sorted_jobs(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60],
        );

        assert_eq!(max_profit_memoization(&jobs), 150);
    }

    #[test]
    fn test_max_profit_memoization_3() {
        let jobs = get_sorted_jobs(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]);

        assert_eq!(max_profit_memoization(&jobs), 6);
    }

    #[test]
    fn test_max_profit_tabulation_1() {
        let jobs = get_sorted_jobs(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]);

        assert_eq!(max_profit_tabulation(&jobs), 120);
    }

    #[test]
    fn test_max_profit_tabulation_2() {
        let jobs = get_sorted_jobs(
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60],
        );

        assert_eq!(max_profit_tabulation(&jobs), 150);
    }

    #[test]
    fn test_max_profit_tabulation_3() {
        let jobs = get_sorted_jobs(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]);

        assert_eq!(max_profit_tabulation(&jobs), 6);
    }
}
