//! This crate aims to provide implementations in Rust of several data structures using linked lists as their underlying data structure
//!
//! This crate’s documentation provides some simple examples and extensively details the exposed API
//!
//! # Usage
//! This crate is on [crates.io](https://crates.io/crates/rust_linked_list) and can be used by adding linked_lists_rs to your dependencies in your project’s Cargo.toml.
//! ```toml
//! [dependencies]
//! linked_lists_rs = "1"
//! ```
//!
//! # Example
//! ```
//! use linked_lists_rs::stack::Stack;
//!
//! let mut stack = Stack::new();
//!
//! stack.push(5);
//!
//! assert_eq!(Some(&5), stack.peek());
//! assert_eq!(Some(5), stack.pop());
//! assert_eq!(None, stack.pop());
//! ```

/// Immutable List implementation
pub mod immutable_list;
/// Queue implementation
pub mod queue;
/// Stack implementation
pub mod stack;
