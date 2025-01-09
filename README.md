# Rust Iterator Invalidation Bug

This repository demonstrates a common error in Rust: invalidating an iterator by modifying the underlying vector while iterating.

The `bug.rs` file contains code that attempts to iterate over a vector and modify it simultaneously. This results in undefined behavior because the iterator's internal pointers become invalidated.

The `bugSolution.rs` file provides a corrected version of the code, showing how to safely iterate and modify vectors.

This example highlights the importance of understanding how iterators work in Rust and how to avoid common pitfalls.