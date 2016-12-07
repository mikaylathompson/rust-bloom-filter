#![cfg_attr(not(test), allow(dead_code))]

use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

// All functions are considered dead_code because they're only called in test build

fn hash_function(s: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(s.as_bytes());
    hasher.finish()
}

struct BloomFilter {
    set: u64,
}

impl BloomFilter {
    fn new() -> BloomFilter {
        BloomFilter{ set: 0 }
    }

    fn add(&self, word: String) -> BloomFilter {
        BloomFilter { set: hash_function(&word) | self.set }
    }

    fn test(&self, word: String) -> bool {
        self.set == (self.set | hash_function(&word))
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn instantiate_bloom_filter() {
    let x = BloomFilter::new();
}

#[test]
fn add_fn_runs() {
    let empty = BloomFilter::new();
    let oneword = empty.add("hello".to_string());
    let twowords = oneword.add("world".to_string());
}

#[test]
fn test_fn_runs() {
    let empty = BloomFilter::new();
    empty.test("hello".to_string());
}

#[test]
fn inserted_words_are_found() {
    let set = BloomFilter::new().add("a".to_string())
                                .add("b".to_string())
                                .add("c".to_string())
                                .add("d".to_string())
                                .add("e".to_string())
                                .add("f".to_string())
                                .add("g".to_string());
    assert!(set.test("a".to_string()));
    assert!(set.test("b".to_string()));
    assert!(set.test("c".to_string()));
    assert!(set.test("d".to_string()));
    assert!(set.test("e".to_string()));
    assert!(set.test("f".to_string()));
    assert!(set.test("g".to_string()));
}

#[test]
fn not_inserted_words_are_rarely_found() {
    let mut set = BloomFilter::new();
    assert!(! (set.test("a".to_string()) ||
               set.test("b".to_string()) ||
               set.test("c".to_string()) ||
               set.test("d".to_string()) ||
               set.test("e".to_string())));

    set = set.add("a".to_string());
    assert!(! (set.test("b".to_string()) ||
               set.test("c".to_string()) ||
               set.test("d".to_string()) ||
               set.test("e".to_string())));

    set = set.add("b".to_string());
    assert!(! (set.test("c".to_string()) ||
               set.test("d".to_string()) ||
               set.test("e".to_string())));

    set = set.add("c".to_string());
    assert!(! (set.test("d".to_string()) ||
               set.test("e".to_string())));

    set = set.add("d".to_string());
    assert!(!set.test("e".to_string()));

    set = set.add("e".to_string());
    assert!(!set.test("z".to_string()));
}
