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
        Self::insert_recursive(&mut self.root, value);
    }

    fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            None => {
                *node = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
            Some(n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => Self::insert_recursive(&mut n.left, value),
                    Ordering::Greater => Self::insert_recursive(&mut n.right, value),
                    Ordering::Equal => {} // Duplicate, do nothing
                }
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        Self::contains_recursive(&self.root, value)
    }

    fn contains_recursive(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => {
                match value.cmp(&n.value) {
                    Ordering::Equal => true,
                    Ordering::Less => Self::contains_recursive(&n.left, value),
                    Ordering::Greater => Self::contains_recursive(&n.right, value),
                }
            }
        }
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
