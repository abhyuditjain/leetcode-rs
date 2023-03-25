//! 84. Largest Rectangle in Histogram
//!
//! Hard
//!
//! Given an array of integers heights representing the histogram's bar height where the width of each bar is 1,
//! return the area of the largest rectangle in the histogram.
//!
//! Example 1:
//! Input: heights = [2,1,5,6,2,3]
//! Output: 10
//! Explanation: The above is a histogram where width of each bar is 1.
//! The largest rectangle is shown in the red area, which has an area = 10 units.
//!
//! Example 2:
//! Input: heights = [2,4]
//! Output: 4
//!
//! Constraints:
//! 1 <= heights.length <= 10^5
//! 0 <= heights[i] <= 10^4

pub enum Algorithm {
    Cubic,
    Quadratic,
    LogLinear,
    Linear,
}

pub fn area_of_largest_rectangle(heights: &[usize], alg: Algorithm) -> usize {
    match alg {
        Algorithm::Cubic => area_cubic(heights),
        Algorithm::Quadratic => area_quadratic(heights),
        Algorithm::LogLinear => area_log_linear(heights),
        Algorithm::Linear => area_linear(heights),
    }
}

pub fn area_linear(heights: &[usize]) -> usize {
    let mut max_area = 0;

    let mut stack: Vec<(usize, usize)> = vec![];

    for (i, &h) in heights.iter().enumerate() {
        let mut start = i;

        while !stack.is_empty() && stack.last().unwrap().1 > heights[i] {
            let (idx, height) = stack.pop().unwrap();
            let area = height * (i - idx);

            max_area = max_area.max(area);

            start = idx;
        }

        stack.push((start, h));
    }

    while !stack.is_empty() {
        let (idx, height) = stack.pop().unwrap();
        let area = height * (heights.len() - idx);

        max_area = max_area.max(area);
    }

    max_area
}

pub fn area_log_linear(heights: &[usize]) -> usize {
    area_log_linear_helper(heights, 0, heights.len() - 1)
}

pub fn area_log_linear_helper(heights: &[usize], start: usize, end: usize) -> usize {
    if start > end {
        return 0;
    }

    let (i, min_height) = heights
        .iter()
        .enumerate()
        .skip(start)
        .take(end - start + 1)
        .min_by_key(|&(_, h)| *h)
        .unwrap();

    let area_using_min_height = min_height * (end - start + 1);
    let area_left_of_min_height = area_log_linear_helper(heights, i + 1, end);
    let area_right_of_min_height = if i > 0 {
        area_log_linear_helper(heights, start, i - 1)
    } else {
        0
    };

    area_using_min_height.max(area_left_of_min_height.max(area_right_of_min_height))
}

pub fn area_quadratic(heights: &[usize]) -> usize {
    let mut max_area = 0;
    for i in 0..heights.len() {
        let mut min_height = heights[i];
        for (j, &height) in heights.iter().enumerate().skip(i) {
            min_height = min_height.min(height);

            max_area = max_area.max(min_height * (j - i + 1));
        }
    }
    max_area
}

pub fn area_cubic(heights: &[usize]) -> usize {
    let mut max_area = 0;

    for i in 0..heights.len() {
        for j in i..heights.len() {
            let min_height = heights.iter().skip(i).take(j - i + 1).min().unwrap();

            max_area = max_area.max(min_height * (j - i + 1));
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_of_largest_rectangle_linear() {
        assert_eq!(area_linear(&[2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(area_linear(&[2, 4]), 4);
    }

    #[test]
    fn test_area_of_largest_rectangle_log_linear() {
        assert_eq!(area_log_linear(&[2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(area_log_linear(&[2, 4]), 4);
        assert_eq!(area_log_linear(&[1, 1, 1, 1]), 4);
    }

    #[test]
    fn test_area_of_largest_rectangle_quadratic() {
        assert_eq!(area_quadratic(&[2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(area_quadratic(&[2, 4]), 4);
    }

    #[test]
    fn test_area_of_largest_rectangle_cubic() {
        assert_eq!(area_cubic(&[2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(area_cubic(&[2, 4]), 4);
    }
}
