# rust-bloom-filter
Rust bloom filter implementation


# Usage

To use this crate in your project, add it as a dependency in your Cargo.toml file:

```Rust

[dependencies]

rust-bloom-filter = "0.1.0"


extern crate my_crate;

use rust-bloom-filter::BloomFilter;

fn main() {
    let mut bloom_filter = BloomFilter::new(1000);
    bloom_filter.add(&"hello");
    bloom_filter.add(&"world");
    assert!(bloom_filter.might_contain(&"hello"));
    assert!(bloom_filter.might_contain(&"world"));
    assert!(!bloom_filter.might_contain(&"goodbye"));
}
