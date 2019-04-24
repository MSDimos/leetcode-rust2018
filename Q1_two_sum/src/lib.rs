use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, usize> = HashMap::new();


        for (index, value) in nums.into_iter().enumerate() {
            let diff = target - value;

            if hash_map.contains_key(&diff) {
                return vec![hash_map.remove(&diff).unwrap() as i32, index as i32];
            } else {
                hash_map.insert(value, index);
            }
        }

        vec![]
    }
}