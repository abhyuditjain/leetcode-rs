use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug, Default)]
pub struct Trie<T> {
    children: HashMap<T, Trie<T>>,
    is_end: bool,
}

impl<T> Trie<T> {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

impl<T, A> FromIterator<T> for Trie<A>
where
    T: IntoIterator<Item = A>,
    A: Eq + Hash + Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut trie = Self::new();

        for it in iter {
            trie.insert(it);
        }

        trie
    }
}

impl<T> Trie<T>
where
    T: Hash + Eq + Debug,
{
    pub fn insert(&mut self, word: impl IntoIterator<Item = T>) {
        let mut curr = self;

        for c in word {
            curr = curr.children.entry(c).or_insert(Trie::new());
        }

        curr.is_end = true;
    }

    pub fn contains_with_deviations(&self, word: &[T], max_deviations: isize) -> bool {
        Self::search_with_allowed_deviations(self, word, 0, max_deviations)
    }

    pub fn contains(&self, word: impl IntoIterator<Item = T>) -> bool {
        let mut curr = self;

        for c in word {
            curr = match curr.children.get(&c) {
                Some(curr) => curr,
                None => return false,
            };
        }

        curr.is_end
    }

    pub fn contains_prefix(&self, word: impl IntoIterator<Item = T>) -> bool {
        let mut curr = self;

        for c in word {
            curr = match curr.children.get(&c) {
                Some(curr) => curr,
                None => return false,
            };
        }

        true
    }

    fn search_with_allowed_deviations(
        node: &Trie<T>,
        word: &[T],
        i: usize,
        remaining_deviations: isize,
    ) -> bool {
        if i == word.len() {
            return node.is_end && remaining_deviations == 0;
        }

        let mut result = false;

        let c = &word[i];

        for (k, child) in &node.children {
            if k != c && remaining_deviations > 0 {
                result = result
                    || Self::search_with_allowed_deviations(
                        child,
                        word,
                        i + 1,
                        remaining_deviations - 1,
                    );
            }

            if !result && k == c {
                result =
                    Self::search_with_allowed_deviations(child, word, i + 1, remaining_deviations);
            }
        }

        result
    }
}

impl<T> Trie<T>
where
    T: Hash + Eq + Clone,
{
    pub fn get_all_with_prefix(&self, prefix: impl IntoIterator<Item = T>) -> Vec<Vec<T>> {
        let mut results = vec![];
        let mut prefix = prefix.into_iter();
        Self::dfs(&mut results, &mut vec![], &mut prefix, self);

        results
    }

    fn dfs(
        results: &mut Vec<Vec<T>>,
        current: &mut Vec<T>,
        prefix: &mut impl Iterator<Item = T>,
        node: &Trie<T>,
    ) {
        if node.is_end {
            results.push(current.clone());
        }
        match prefix.next() {
            Some(c) => {
                if let Some(child) = node.children.get(&c) {
                    current.push(c);
                    Self::dfs(results, current, prefix, child);
                }
            }
            None => {
                for (k, v) in &node.children {
                    current.push(k.clone());
                    Self::dfs(results, current, prefix, v);
                    current.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let mut trie: Trie<char> = Trie::new();

        trie.insert("hello".chars());

        assert!(trie.contains("hello".chars()));
        assert!(!trie.contains("hell".chars()));
    }

    #[test]
    fn test_contains_prefix() {
        let mut trie: Trie<char> = Trie::new();

        trie.insert("hello".chars());

        assert!(trie.contains_prefix("hello".chars()));
        assert!(trie.contains_prefix("hell".chars()));
        assert!(trie.contains_prefix("hel".chars()));
        assert!(trie.contains_prefix("he".chars()));
        assert!(trie.contains_prefix("h".chars()));
        assert!(!trie.contains_prefix("a".chars()));
        assert!(!trie.contains_prefix("ello".chars()));
    }

    #[test]
    fn test_get_all_with_prefix() {
        let mut trie: Trie<char> = Trie::new();

        trie.insert("hell".chars());
        trie.insert("hello".chars());
        trie.insert("how".chars());
        trie.insert("exit".chars());

        assert_eq!(
            trie.get_all_with_prefix("e".chars()),
            vec![vec!['e', 'x', 'i', 't']]
        );

        let mut res = trie.get_all_with_prefix("h".chars());
        res.sort_unstable();
        assert_eq!(
            res,
            vec![
                vec!['h', 'e', 'l', 'l'],
                vec!['h', 'e', 'l', 'l', 'o'],
                vec!['h', 'o', 'w'],
            ]
        );

        let mut res = trie.get_all_with_prefix("".chars());
        res.sort_unstable();

        assert_eq!(
            res,
            vec![
                vec!['e', 'x', 'i', 't'],
                vec!['h', 'e', 'l', 'l'],
                vec!['h', 'e', 'l', 'l', 'o'],
                vec!['h', 'o', 'w'],
            ]
        );
    }

    #[test]
    fn test_from_iter() {
        let iter = vec!["hello".chars(), "exit".chars(), "how".chars()].into_iter();

        let trie = Trie::from_iter(iter);

        assert!(trie.contains("hello".chars()));
        assert!(trie.contains("exit".chars()));
        assert!(trie.contains("how".chars()));
    }

    #[test]
    fn test_contains_with_deviations() {
        let iter = vec!["hello".chars(), "exit".chars(), "how".chars()].into_iter();

        let trie = Trie::from_iter(iter);

        assert!(trie.contains_with_deviations(&['h', 'e', 'l', 'l', 'o'], 0));
        assert!(trie.contains_with_deviations(&['h', 'w', 'l', 'l', 'o'], 1));
        assert!(trie.contains_with_deviations(&['h', 'w', 'w', 'l', 'o'], 2));
    }
}
