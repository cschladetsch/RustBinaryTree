# RustBinaryTree 

A high-performance, memory-efficient, and generic Binary Search Tree (BST) implementation in modern Rust, engineered for systems programming, low-latency applications, and extensible tree-based data structures.

## Features

* **Generic & Flexible**: Supports any element type implementing `Ord` and `Clone`.
* **Zero-Cost Abstractions**: Idiomatic Rust design leveraging ownership, lifetimes, and safe memory management.
* **Comprehensive Traversals**: Built-in support for In-Order, Pre-Order, Post-Order, and Level-Order (Breadth-First Search) traversals.
* **Rich Iterators**: Iterators for consuming (`into_iter`), borrowing (`iter`), and mutably borrowing (`iter_mut`) tree nodes.
* **Robust Test Suite**: Includes 50 rigorous unit and integration tests covering edge cases, balance properties, node deletions, search queries, and stress tests.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust_binary_tree = { version = "0.1.0", git = "https://github.com/cschladetsch/RustBinaryTree.git" }
```

---

## Quick Start

```rust
use rust_binary_tree::BinarySearchTree;

fn main() {
    let mut bst = BinarySearchTree::new();
    
    // Insert elements
    bst.insert(50);
    bst.insert(30);
    bst.insert(70);
    bst.insert(20);
    bst.insert(40);

    // Search for elements
    assert!(bst.contains(&40));

    // In-order traversal
    let sorted: Vec<_> = bst.in_order().collect();
    println!("Sorted elements: {:?}", sorted); // [20, 30, 40, 50, 70]
}
```

---

## API Overview

### Construction & Management

| Method | Signature | Description | Time Complexity |
| :--- | :--- | :--- | :--- |
| `new` | `pub fn new() -> Self` | Creates an empty binary search tree. | $O(1)$ |
| `default` | `pub fn default() -> Self` | Creates a default empty tree. | $O(1)$ |
| `clear` | `pub fn clear(&mut self)` | Removes all nodes from the tree. | $O(n)$ |
| `len` | `pub fn len(&self) -> usize` | Returns the number of nodes in the tree. | $O(1)$ |
| `is_empty` | `pub fn is_empty(&self) -> bool` | Returns `true` if the tree contains no nodes. | $O(1)$ |

### Insertion & Modification

| Method | Signature | Description | Time Complexity |
| :--- | :--- | :--- | :--- |
| `insert` | `pub fn insert(&mut self, value: T) -> bool` | Inserts a value into the tree; returns `true` if newly added. | $O(\log n)$ avg |
| `remove` | `pub fn remove(&mut self, value: &T) -> bool` | Removes a value from the tree; returns `true` if found and removed. | $O(\log n)$ avg |

### Querying & Statistics

| Method | Signature | Description | Time Complexity |
| :--- | :--- | :--- | :--- |
| `contains` | `pub fn contains(&self, value: &T) -> bool` | Checks if a value exists in the tree. | $O(\log n)$ avg |
| `min` | `pub fn min(&self) -> Option<&T>` | Returns a reference to the minimum value. | $O(\log n)$ avg |
| `max` | `pub fn max(&self) -> Option<&T>` | Returns a reference to the maximum value. | $O(\log n)$ avg |
| `height` | `pub fn height(&self) -> usize` | Returns the maximum depth of the tree. | $O(n)$ |
| `is_balanced`| `pub fn is_balanced(&self) -> bool` | Checks if the tree satisfies height-balance properties. | $O(n)$ |
| `successor`| `pub fn successor(&self, val: &T) -> Option<&T>`| Finds the in-order successor of a value. | $O(\log n)$ avg |
| `predecessor`| `pub fn predecessor(&self, val: &T) -> Option<&T>`| Finds the in-order predecessor of a value. | $O(\log n)$ avg |

### Iterators & Traversals

| Method | Description |
| :--- | :--- |
| `in_order` | Returns an iterator visiting nodes in ascending sorted order. |
| `pre_order` | Returns an iterator visiting nodes in pre-order (root, left, right). |
| `post_order` | Returns an iterator visiting nodes in post-order (left, right, root). |
| `level_order`| Returns an iterator visiting nodes level-by-level (breadth-first). |
| `iter` | Borrows elements immutably. |
| `iter_mut` | Borrows elements mutably. |
| `into_iter` | Consumes the tree, yielding owned elements. |

---

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
