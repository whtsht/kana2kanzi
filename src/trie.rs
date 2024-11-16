use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    probability: Option<f64>,
}

#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, first: &str, second: &str, probability: f64) {
        let mut node = &mut self.root;

        for c in first.chars() {
            node = node.children.entry(c).or_default();
        }

        for c in second.chars() {
            node = node.children.entry(c).or_default();
        }

        node.probability = Some(probability);
    }

    pub fn search(&self, first: &str, second: &str) -> Option<f64> {
        let mut node = &self.root;

        for c in first.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return None;
            }
        }

        for c in second.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return None;
            }
        }

        node.probability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();

        trie.insert("に", "場合", 0.2000);
        trie.insert("場合", "という", 0.1111);
        trie.insert("が", "場合", 0.4000);
        trie.insert("は", "場合", 0.1000);

        assert_eq!(trie.search("に", "場合"), Some(0.2000));
        assert_eq!(trie.search("場合", "という"), Some(0.1111));
        assert_eq!(trie.search("が", "場合"), Some(0.4000));
        assert_eq!(trie.search("は", "場合"), Some(0.1000));
    }

    #[test]
    fn test_search_not_found() {
        let mut trie = Trie::new();

        trie.insert("に", "場合", 0.2000);
        trie.insert("場合", "という", 0.1111);

        assert_eq!(trie.search("に", "という"), None);
        assert_eq!(trie.search("が", "場合"), None);
    }

    #[test]
    fn test_multiple_insertions() {
        let mut trie = Trie::new();

        trie.insert("に", "場合", 0.2000);
        trie.insert("に", "場合", 0.3000);

        assert_eq!(trie.search("に", "場合"), Some(0.3000));
    }

    #[test]
    fn test_edge_cases() {
        let mut trie = Trie::new();

        trie.insert("", "場合", 0.1000);
        assert_eq!(trie.search("", "場合"), Some(0.1000));

        trie.insert("に", "", 0.2000);
        assert_eq!(trie.search("に", ""), Some(0.2000));
    }

    #[test]
    fn test_search_non_inserted() {
        let trie = Trie::new();
        assert_eq!(trie.search("非", "挿入"), None);
    }

    #[test]
    fn test_search_nested() {
        let mut trie = Trie::new();
        trie.insert("あ", "お", 0.1000);
        trie.insert("あい", "お", 0.2000);
        trie.insert("あいう", "お", 0.3000);
        trie.insert("あいうえ", "お", 0.4000);

        assert_eq!(trie.search("あ", "お"), Some(0.1000));
        assert_eq!(trie.search("あい", "お"), Some(0.2000));
        assert_eq!(trie.search("あいう", "お"), Some(0.3000));
        assert_eq!(trie.search("あいうえ", "お"), Some(0.400));
    }
}
