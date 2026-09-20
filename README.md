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
rust_binary_tree = { version = "0.1.0", git = "[https://github.com/cschladetsch/RustBinaryTree.git](https://github.com/cschladetsch/RustBinaryTree.git)" }