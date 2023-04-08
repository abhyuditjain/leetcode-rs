pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![1; n],
        }
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let parent_a = self.find(a);
        let parent_b = self.find(b);

        if parent_a == parent_b {
            return false;
        }

        if self.ranks[parent_a] >= self.ranks[parent_b] {
            self.parents[parent_b] = parent_a;
            self.ranks[parent_a] += self.ranks[parent_b];
        } else {
            self.parents[parent_a] = parent_b;
            self.ranks[parent_b] += self.ranks[parent_a];
        }

        true
    }

    pub fn find(&mut self, a: usize) -> usize {
        if a == self.parents[a] {
            return a;
        }

        // path compression
        self.parents[a] = self.find(self.parents[a]);

        self.parents[a]
    }

    pub fn is_connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let mut uf = UnionFind::new(5);

        assert!(!uf.union(0, 0));
        assert!(!uf.union(1, 1));
        assert!(!uf.union(2, 2));
        assert!(!uf.union(3, 3));
        assert!(!uf.union(4, 4));

        assert!(uf.union(0, 1));
        assert!(uf.union(0, 2));
        assert!(uf.union(0, 3));
        assert!(uf.union(0, 4));

        assert!(!uf.union(1, 0));
        assert!(!uf.union(1, 2));
        assert!(!uf.union(1, 3));
        assert!(!uf.union(1, 4));

        assert!(!uf.union(2, 0));
        assert!(!uf.union(2, 1));
        assert!(!uf.union(2, 3));
        assert!(!uf.union(2, 4));

        assert!(!uf.union(3, 0));
        assert!(!uf.union(3, 1));
        assert!(!uf.union(3, 2));
        assert!(!uf.union(3, 4));
    }

    #[test]
    fn test_find() {
        let mut uf = UnionFind::new(5);

        // Initially every node is its own parent
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 4);

        // Since ranks of 0 and 1 are same (=1), 0 becomes the parent of 1
        // Rank of 0 becomes 2
        uf.union(0, 1);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 0);

        // Since ranks of 2 and 3 are same (=1), 2 becomes the parent of 3
        // Rank of 2 becomes 2
        uf.union(2, 3);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(3), 2);

        // Since rank of 2 = 2 and rank of 4 = 1, 2 becomes the parent of 4
        // Rank of 2 becomes 3
        uf.union(2, 4);
        assert_eq!(uf.find(4), 2);

        // Since rank of 2 = 3 and rank of 0 = 2, 2 becomes the parent of 0 and all its connected nodes
        // Rank of 2 becomes 3 + 2 = 5
        uf.union(0, 2);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(0), 2);
        assert_eq!(uf.find(1), 2);
        assert_eq!(uf.find(3), 2);
        assert_eq!(uf.find(4), 2);
    }

    #[test]
    fn test_is_connected() {
        let mut uf = UnionFind::new(3);

        assert!(uf.is_connected(0, 0));
        assert!(uf.is_connected(1, 1));
        assert!(uf.is_connected(2, 2));

        assert!(!uf.is_connected(0, 1));
        assert!(!uf.is_connected(0, 2));
        assert!(!uf.is_connected(1, 2));

        uf.union(0, 1);
        assert!(uf.is_connected(0, 1));
        assert!(!uf.is_connected(1, 2));

        uf.union(1, 2);
        assert!(uf.is_connected(0, 1));
        assert!(uf.is_connected(1, 2));
        assert!(uf.is_connected(0, 2));
    }
}
