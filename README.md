# minimizer-queue

[![crates.io](https://img.shields.io/crates/v/minimizer-queue)](https://crates.io/crates/minimizer-queue)
[![docs](https://img.shields.io/docsrs/minimizer-queue)](https://docs.rs/minimizer-queue)

Fast computation of minimizers using a monotone queue.

## Features

- insertion in amortized constant time
- lookup in constant time
- keeps track of the relative position of the minimizers
- supports custom [hasher](https://doc.rust-lang.org/stable/core/hash/trait.BuildHasher.html), using [wyhash](https://github.com/JackThomson2/wyhash2) by default
- can be seeded to produce a different ordering
- optimized modulo computation with [strength_reduce](https://github.com/ejmahler/strength_reduce)

## Example usage

```rust
use minimizer_queue::MinimizerQueue;

let mut queue = MinimizerQueue::new(3); // width 3
queue.insert(1);
queue.insert(2);
queue.insert(3);
queue.get_min(); // element with the smallest hash among 1, 2 and 3

queue.insert(4);
queue.get_min(); // element with the smallest hash among 2, 3 and 4
```
