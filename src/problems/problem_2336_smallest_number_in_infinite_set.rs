//! 2336. Smallest Number in Infinite Set
//!
//! Medium
//!
//! You have a set which contains all positive integers [1, 2, 3, 4, 5, ...].
//! Implement the SmallestInfiniteSet class:
//! SmallestInfiniteSet() Initializes the SmallestInfiniteSet object to contain all positive integers.
//! int popSmallest() Removes and returns the smallest integer contained in the infinite set.
//! void addBack(int num) Adds a positive integer num back into the infinite set, if it is not already in the infinite set.
//!
//! Example 1:
//! Input
//! ["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
//! [[], [2], [], [], [], [1], [], [], []]
//! Output
//! [null, null, 1, 2, 3, null, 1, 4, 5]
//! Explanation
//! SmallestInfiniteSet smallestInfiniteSet = new SmallestInfiniteSet();
//! smallestInfiniteSet.addBack(2);    // 2 is already in the set, so no change is made.
//! smallestInfiniteSet.popSmallest(); // return 1, since 1 is the smallest number, and remove it from the set.
//! smallestInfiniteSet.popSmallest(); // return 2, and remove it from the set.
//! smallestInfiniteSet.popSmallest(); // return 3, and remove it from the set.
//! smallestInfiniteSet.addBack(1);    // 1 is added back to the set.
//! smallestInfiniteSet.popSmallest(); // return 1, since 1 was added back to the set and
//!                                    // is the smallest number, and remove it from the set.
//! smallestInfiniteSet.popSmallest(); // return 4, and remove it from the set.
//! smallestInfiniteSet.popSmallest(); // return 5, and remove it from the set.
//!
//! Constraints:
//! 1 <= num <= 1000
//! At most 1000 calls will be made in total to popSmallest and addBack.

use std::collections::BTreeSet;

pub struct SmallestInfiniteSet {
    current: i32,
    set: BTreeSet<i32>,
}

impl Default for SmallestInfiniteSet {
    fn default() -> Self {
        Self {
            current: 1,
            set: Default::default(),
        }
    }
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if let Some(smallest) = self.set.pop_first() {
            return smallest;
        }

        self.current += 1; // increment the current integer

        self.current - 1 // return the previous integer
    }

    pub fn add_back(&mut self, num: i32) {
        if num < self.current {
            self.set.insert(num);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_infinite_set() {
        let mut s = SmallestInfiniteSet::new();

        s.add_back(2);

        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 2);
        assert_eq!(s.pop_smallest(), 3);

        s.add_back(1);

        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 4);
        assert_eq!(s.pop_smallest(), 5);
    }
}
