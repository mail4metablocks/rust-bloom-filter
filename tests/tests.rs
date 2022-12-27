use my_crate::BloomFilter;

#[test]
fn test_bloom_filter() {
    let mut bloom_filter = BloomFilter::new(1000);
    bloom_filter.add(&"hello");
    bloom_filter.add(&"world");
    assert!(bloom_filter.might_contain(&"hello"));
    assert!(bloom_filter.might_contain(&"world"));
    assert!(!bloom_filter.might_contain(&"goodbye"));
}
