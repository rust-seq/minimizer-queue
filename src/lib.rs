use ahash::RandomState;
use core::hash::{BuildHasher, Hash};
use std::collections::VecDeque;
use strength_reduce::StrengthReducedU16;

pub struct MinimizerQueue<T: Hash + Copy, S: BuildHasher = RandomState> {
    deq: VecDeque<(T, u64, u16)>,
    width: StrengthReducedU16,
    hash_builder: S,
    pos: u16,
}

impl<T: Hash + Copy> MinimizerQueue<T> {
    #[inline]
    pub fn new(width: usize) -> Self {
        Self::with_seed(width, width)
    }

    #[inline]
    pub fn with_seed(width: usize, seed: usize) -> Self {
        Self::with_hasher(width, RandomState::with_seed(seed))
    }
}

impl<T: Hash + Copy, S: BuildHasher> MinimizerQueue<T, S> {
    pub fn with_hasher(width: usize, hash_builder: S) -> Self {
        debug_assert!(width <= u16::MAX as usize, "Width must fit in a u16");
        Self {
            deq: VecDeque::with_capacity(width),
            width: StrengthReducedU16::new(width as u16),
            hash_builder,
            pos: 0,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width.get() as usize
    }

    #[inline]
    pub fn get_min(&self) -> T {
        debug_assert!(!self.deq.is_empty(), "MinimizerQueue is empty");
        self.deq[0].0
    }

    #[inline]
    pub fn get_min_pos(&self) -> (T, usize) {
        debug_assert!(!self.deq.is_empty(), "MinimizerQueue is empty");
        let (x, _, pos) = self.deq[0];
        let rel_pos = ((self.width.get() - self.pos + pos) % self.width) as usize;
        (x, rel_pos)
    }

    #[inline]
    pub fn insert(&mut self, x: T) {
        self.insert_with_hash(x, self.hash_builder.hash_one(x))
    }

    pub fn insert_with_hash(&mut self, x: T, hash: u64) {
        dbg!(hash);
        if !self.deq.is_empty() && self.deq[0].2 == self.pos {
            self.deq.pop_front();
        }
        let mut i = self.deq.len();
        while i > 0 && hash < self.deq[i - 1].1 {
            i -= 1;
        }
        self.deq.truncate(i);
        self.deq.push_back((x, hash, self.pos));
        self.pos = (self.pos + 1) % self.width;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nohash_hasher::BuildNoHashHasher;

    #[test]
    fn test_get_min() {
        let width = 3;
        let mut queue = MinimizerQueue::with_hasher(width, BuildNoHashHasher::<usize>::default());

        let vals = [1usize, 2, 3, 0, 7, 8, 9, 100, 3, 4, 7, 8];
        let mut mins = Vec::with_capacity(vals.len() - width + 1);

        for &val in vals.iter().take(width - 1) {
            queue.insert(val);
        }
        for &val in vals.iter().skip(width - 1) {
            queue.insert(val);
            mins.push(queue.get_min());
        }

        assert_eq!(mins, vec![1, 0, 0, 0, 7, 8, 3, 3, 3, 4]);
    }

    #[test]
    fn test_get_min_pos() {
        let width = 3;
        let mut queue = MinimizerQueue::with_hasher(width, BuildNoHashHasher::<usize>::default());

        let vals = [1usize, 2, 3, 0, 7, 8, 9, 100, 3, 4, 7, 8];
        let mut mins_pos = Vec::with_capacity(vals.len() - width + 1);

        for &val in vals.iter().take(width - 1) {
            queue.insert(val);
        }
        for &val in vals.iter().skip(width - 1) {
            queue.insert(val);
            mins_pos.push(queue.get_min_pos());
        }

        assert_eq!(
            mins_pos,
            vec![
                (1, 0),
                (0, 2),
                (0, 1),
                (0, 0),
                (7, 0),
                (8, 0),
                (3, 2),
                (3, 1),
                (3, 0),
                (4, 0)
            ]
        );
    }
}
