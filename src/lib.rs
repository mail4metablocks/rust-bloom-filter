use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const NUM_HASHES: usize = 3;

struct BloomFilter {
    bits: Vec<bool>,
    num_hashes: usize,
}

impl BloomFilter {
    fn new(size: usize) -> Self {
        Self {
            bits: vec![false; size],
            num_hashes: NUM_HASHES,
        }
    }

    fn add<T: Hash>(&mut self, item: &T) {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        for i in 0..self.num_hashes {
            let index = (hash as usize + i) % self.bits.len();
            self.bits[index] = true;
        }
    }

    fn might_contain<T: Hash>(&self, item: &T) -> bool {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        for i in 0..self.num_hashes {
            let index = (hash as usize + i) % self.bits.len();
            if !self.bits[index] {
                return false;
            }
        }
        true
    }
}
