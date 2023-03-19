//! 42. Trapping Rain Water
//!
//! Hard
//!
//! Given n non-negative integers representing an elevation map where the width of each bar is 1,
//! compute how much water it can trap after raining.
//!
//! Example 1:
//! Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
//! Output: 6
//! Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1].
//! In this case, 6 units of rain water (blue section) are being trapped.
//!
//! Example 2:
//! Input: height = [4,2,0,3,2,5]
//! Output: 9
//!
//! Constraints:
//! n == height.length
//! 1 <= n <= 2 * 104
//! 0 <= height[i] <= 105

pub fn trap_rain_water(heights: Vec<usize>, use_space: bool) -> usize {
    if use_space {
        trap_rain_water_with_extra_space(&heights)
    } else {
        trap_rain_water_no_extra_space(&heights)
    }
}

fn trap_rain_water_with_extra_space(heights: &[usize]) -> usize {
    let mut max_heights_left = vec![0; heights.len()];
    let mut max_heights_right = vec![0; heights.len()];

    for i in 1..heights.len() {
        let j = heights.len() - i - 1;

        max_heights_left[i] = max_heights_left[i - 1].max(heights[i - 1]);
        max_heights_right[j] = max_heights_right[j + 1].max(heights[j + 1]);
    }

    heights
        .iter()
        .zip(max_heights_left.iter())
        .zip(max_heights_right.iter())
        .map(|((height, max_left), max_right)| max_left.min(max_right).saturating_sub(*height))
        .sum()
}

fn trap_rain_water_no_extra_space(heights: &[usize]) -> usize {
    let mut max_height_left = 0;
    let mut max_height_right = 0;

    let mut left = 0;
    let mut right = heights.len() - 1;

    let mut total = 0;

    while left <= right {
        max_height_left = max_height_left.max(heights[left]);
        max_height_right = max_height_right.max(heights[right]);

        if max_height_left <= max_height_right {
            total += max_height_left - heights[left];
            left += 1;
        } else {
            total += max_height_right - heights[right];
            right -= 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_rain_water_with_extra_space() {
        assert_eq!(
            trap_rain_water_with_extra_space(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(trap_rain_water_with_extra_space(&[4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_trap_rain_water_no_extra_space() {
        assert_eq!(
            trap_rain_water_no_extra_space(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
        assert_eq!(trap_rain_water_no_extra_space(&[4, 2, 0, 3, 2, 5]), 9);
    }
}
