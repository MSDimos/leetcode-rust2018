use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut hash_set: HashMap<i32, i32> = HashMap::new();

        for num in nums.into_iter() {
            if hash_set.contains_key(&num) {
                continue;
            }

            let left_part = *hash_set.get(&(num - 1)).unwrap_or(&0);
            let right_part = *hash_set.get(&(num + 1)).unwrap_or(&0);

            let len = left_part + right_part + 1;
            max_len = max(max_len, len);
            hash_set.insert(num, len);
            hash_set.insert(num - left_part, len);
            hash_set.insert(num + right_part, len);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 0, 1]), 3);
    }
}
