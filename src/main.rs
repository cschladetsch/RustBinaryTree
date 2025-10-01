use std::cmp::Ordering;

// Node structure using Box for heap allocation
// Box<T> is like unique_ptr<T> in C++
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Binary Search Tree
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        // TODO: Implement insertion
        // Challenge: How do you traverse and mutate the tree
        // without violating Rust's ownership rules?
    }

    pub fn contains(&self, value: &T) -> bool {
        // TODO: Implement search
        // This is easier than insert - immutable traversal
        false
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    // Test values
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    println!("Tree contains 7: {}", tree.contains(&7));
    println!("Tree contains 4: {}", tree.contains(&4));
}
