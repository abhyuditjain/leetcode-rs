//! 946. Validate Stack Sequences
//!
//! Medium
//!
//! Given two integer arrays pushed and popped each with distinct values, return true if this could have been the result of a sequence of push and pop operations on an initially empty stack, or false otherwise.
//!
//! Example 1:
//! Input: pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
//! Output: true
//! Explanation: We might do the following sequence:
//! push(1), push(2), push(3), push(4),
//! pop() -> 4,
//! push(5),
//! pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
//!
//! Example 2:
//! Input: pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
//! Output: false
//! Explanation: 1 cannot be popped before 2.
//!
//! Constraints:
//! 1 <= pushed.length <= 1000
//! 0 <= pushed[i] <= 1000
//! All the elements of pushed are unique.
//! popped.length == pushed.length
//! popped is a permutation of pushed.

pub fn is_stack_sequence_valid(pushed: &[i32], popped: &[i32]) -> bool {
    let mut stack = Vec::with_capacity(pushed.len());
    let mut popped_idx = 0;

    for &v in pushed {
        stack.push(v);

        while stack.last().map_or(false, |top| *top == popped[popped_idx]) {
            stack.pop();
            popped_idx += 1;
        }
    }

    popped_idx == popped.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_stack_sequence_valid() {
        assert!(is_stack_sequence_valid(&[1, 2, 3, 4, 5], &[4, 5, 3, 2, 1]));
        assert!(!is_stack_sequence_valid(&[1, 2, 3, 4, 5], &[4, 3, 5, 1, 2]));
    }
}
