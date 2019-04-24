use std::collections::{HashSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 解法1
//        let chars: Vec<char> = s.chars().collect();
//        let n = chars.len();
//        let mut set: HashSet<char> = HashSet::new();
//        let (mut ans, mut i, mut j): (usize, usize, usize) = (0, 0, 0);
//
//        while (i < n && j < n) {
//            if !set.contains(&chars[j]) {
//                set.insert(chars[j]);
//                j += 1;
//                ans = ans.max(j - i);
//            } else {
//                set.remove(&chars[i]);
//                i += 1;
//            }
//        }
//
//        ans as i32
    // 解法二
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut map: HashMap<char, usize> = HashMap::new();
        let (mut ans, mut i, mut j): (usize, usize, usize) = (0, 0, 0);

        for (j, v) in chars.into_iter().enumerate() {
            if map.contains_key(&v) {
                i = (map.remove(&v).unwrap() + 1).max(i);
            }
            ans = ans.max(j - i + 1);
            map.insert(v, j);
        }

        ans as i32

        // 解法3
        // 把HashMap替换成arr[u8]，因为字符串的编码就这些
    }
}