//! 60. Permutation Sequence
//!
//! Hard
//!
//! The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
//! By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
//! "123"
//! "132"
//! "213"
//! "231"
//! "312"
//! "321"
//!
//! Given n and k, return the kth permutation sequence.
//!
//! Example 1:
//! Input: n = 3, k = 3
//! Output: "213"
//!
//! Example 2:
//! Input: n = 4, k = 9
//! Output: "2314"
//!
//! Example 3:
//! Input: n = 3, k = 1
//! Output: "123"
//!
//! Constraints:
//! 1 <= n <= 9
//! 1 <= k <= n!

pub fn get_kth_permutation(n: usize, k: usize) -> String {
    let mut bucket_size = factorial(n - 1);
    let mut numbers = (1..=n).collect::<Vec<_>>();

    let mut kth_factorial = "".to_string();

    let mut k = k - 1; // 0-based

    loop {
        let idx = k / bucket_size;
        kth_factorial.push_str(&numbers[idx].to_string());
        numbers.remove(idx);

        if numbers.is_empty() {
            break;
        }

        k %= bucket_size;
        bucket_size /= numbers.len();
    }

    kth_factorial
}

fn factorial(n: usize) -> usize {
    (2..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_kth_permutation_1() {
        assert_eq!(&get_kth_permutation(3, 3), "213");
    }

    #[test]
    fn test_get_kth_permutation_2() {
        assert_eq!(&get_kth_permutation(4, 9), "2314");
    }

    #[test]
    fn test_get_kth_permutation_3() {
        assert_eq!(&get_kth_permutation(3, 1), "123");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
    }
}
