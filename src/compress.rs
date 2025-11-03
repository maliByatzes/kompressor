use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Symbol {
    value: String,
    count: i32,
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .count
            .cmp(&self.count)
            .then_with(|| self.value.cmp(&other.value))
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Compress {
    contents: String,
}

impl Compress {
    pub fn new(s: &str) -> Self {
        Compress {
            contents: s.to_owned(),
        }
    }

    pub fn compress(&self) {
        let counter = self.freq_counter();

        let heap = BinaryHeap::from(counter);

        for sym in &heap {
            println!("{:?}", sym);
        }
    }

    fn produce_tree(heap: &mut BinaryHeap<Node<Symbol>>) {
        loop {
            if heap.len() <= 1 {
                break;
            }

            let node1 = heap.pop().unwrap();
            let node2 = heap.pop().unwrap();
            let new_val = node1.value.value + &node2.value.value;
            let new_count = node1.value.count + node2.value.count;

            let new_node = Node {
                value: Symbol {
                    value: new_val,
                    count: new_count,
                },
                left: Subtree { node1 },
                right: Subtree { root: node2 },
            };
        }
    }

    fn freq_counter(&self) -> Vec<Node<Symbol>> {
        let mut temp_counter = HashMap::new();

        for chr in self.contents.chars() {
            temp_counter.entry(chr).and_modify(|e| *e += 1).or_insert(1);
        }

        let counter: Vec<_> = temp_counter
            .iter()
            .map(|(&key, &val)| {
                Node::new(Symbol {
                    value: key.to_string(),
                    count: val,
                })
            })
            .collect();

        counter
    }
}

// TODO: move this somewhere else, actually make it a library
// BST implementation (https://google.github.io/comprehensive-rust/smart-pointers/solution.html)

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node<T: Ord> {
    pub value: T,
    pub left: Subtree<T>,
    pub right: Subtree<T>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn insert(&mut self, value: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(value))),
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.insert(value),
                Ordering::Equal => {}
                Ordering::Greater => n.right.insert(value),
            },
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.has(value),
                Ordering::Equal => true,
                Ordering::Greater => n.right.has(value),
            },
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len(),
        }
    }
}

impl<T: Ord> Ord for Subtree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: Ord> PartialOrd for Subtree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

impl<T: Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .value
            .cmp(&self.value)
            .then_with(|| self.left.cmp(&other.left))
            .then_with(|| self.right.cmp(&other.right))
    }
}

impl<T: Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2);
        assert_eq!(tree.len(), 2);
        tree.insert(3);
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
