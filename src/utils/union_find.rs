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
