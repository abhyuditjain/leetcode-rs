//! 50. Pow(x, n)
//!
//! Medium
//!
//! Implement pow(x, n), which calculates x raised to the power n (i.e., x^n).
//!
//! Example 1:
//! Input: x = 2.00000, n = 10
//! Output: 1024.00000
//!
//! Example 2:
//! Input: x = 2.10000, n = 3
//! Output: 9.26100
//!
//! Example 3:
//! Input: x = 2.00000, n = -2
//! Output: 0.25000
//! Explanation: 2^-2 = 1/22 = 1/4 = 0.25
//!
//! Constraints:
//! -100.0 < x < 100.0
//! -2^31 <= n <= 2^31-1
//! n is an integer.
//! -10^4 <= xn <= 10^4

pub fn pow(x: f64, n: i32) -> f64 {
    match (x, n) {
        (x, _) if x == 0.0 => 0.0,                // Any power of 0 = 0
        (x, _) if x == 1.0 => 1.0,                // Any power of 1 = 1
        (x, n) if x == -1.0 && n % 2 == 0 => 1.0, // Even power of -1 = 1
        (x, _) if x == -1.0 => -1.0,              // Odd power of -1 = -1
        (_, i32::MIN) | (_, i32::MAX) => 0.0,
        (_, 0) => 1.0,
        (x, n) if n.is_negative() => pow(1.0 / x, n.abs()),
        (x, n) => {
            let mut product = x;
            let mut result = 1_f64;

            let mut pow = n;

            while pow > 0 {
                if pow % 2 == 1 {
                    result *= product;
                }

                product *= product;

                pow /= 2;
            }

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        assert!(nearly_equal(pow(2.0, 10), 1024.0));
        assert!(nearly_equal(pow(2.1, 3), 9.26100));
        assert!(nearly_equal(pow(2.0, -2), 0.25000));
        assert!(nearly_equal(pow(-1.0, 2), 1.0));
        assert!(nearly_equal(pow(-1.0, 3), -1.0));
    }

    pub fn nearly_equal(a: f64, b: f64) -> bool {
        let abs_a = a.abs();
        let abs_b = b.abs();
        let diff = (a - b).abs();

        if a == b {
            // Handle infinities.
            true
        } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
            // One of a or b is zero (or both are extremely close to it,) use absolute error.
            diff < (f64::EPSILON * f64::MIN_POSITIVE)
        } else {
            // Use relative error.
            (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
        }
    }
}
