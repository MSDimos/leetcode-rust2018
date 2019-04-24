use std::collections::HashMap;

pub struct Solution;

pub struct RomanMap;

impl RomanMap {
    pub fn new() -> HashMap<u8, i32> {
        let mut hash_map = HashMap::new();
        hash_map.insert(b'I', 1);
        hash_map.insert(b'V', 5);
        hash_map.insert(b'X', 10);
        hash_map.insert(b'L', 50);
        hash_map.insert(b'C', 100);
        hash_map.insert(b'D', 500);
        hash_map.insert(b'M', 1000);

        hash_map
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let len = s.len();
        let bytes = s.into_bytes();
        let hash_map = RomanMap::new();
        let mut idx = 0;
        let mut num = 0;

        for i in 0..len {
            let j = hash_map.get(&bytes[i]);
            if i < len - 1 && j < hash_map.get(&bytes[i + 1]) {
                num -= j.unwrap();
            } else {
                num += j.unwrap();
            }
        }

        num
    }
}