use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut hash_map = HashMap::new();

        if n == 0 {
            return 0
        } else {
            Solution::helper(1, n as usize, &mut hash_map)
        }
    }

    fn helper(start: usize, end: usize, map: &mut HashMap<(usize, usize), i32>) -> i32 {
        let mut count = 0;

        if start > end {
            return 1;
        } else {

            if map.contains_key(&(start, end)) {
                return *map.get(&(start, end)).unwrap();
            }

            for i in start..=end {
                let left_nums = Solution::helper(start, i - 1, map);
                let right_nums = Solution::helper(i + 1, end, map);

                count += left_nums * right_nums;
            }
        }

        map.insert((start, end), count);
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::num_trees(0), 0);
        assert_eq!(Solution::num_trees(1), 1);
        assert_eq!(Solution::num_trees(2), 2);
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(19), 1767263190);
    }
}
