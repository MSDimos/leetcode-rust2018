use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn count(hash_map: &mut HashMap<u8, i32>, bytes: &Vec<u8>) {
        for byte in bytes {
            if hash_map.contains_key(byte) {
                if let Some(count) = hash_map.get_mut(byte) {
                    *count += 1;
                }
            } else {
                hash_map.insert(*byte, 1);
            }
        }
    }

    pub fn min_window(s: String, t: String) -> String {
        // sliding window
        if s.is_empty() || t.is_empty() {
            return "".to_string();
        }

        // let mut s_counts = HashMap::new();
        let mut t_counts = HashMap::new();
        let mut window_counts = HashMap::new();
        let (s_bytes, t_bytes) = (s.into_bytes(), t.into_bytes());
        let (mut left_ptr, mut right_ptr) = (0, 0);
        let mut formed = 0;
        let mut result = (-1, 0, 0);

        // Solution::count(&mut s_counts, &s_bytes);
        Solution::count(&mut t_counts, &t_bytes);

        while right_ptr < s_bytes.len() {
            let right_char = s_bytes[right_ptr];

            if let Some(wc_count) = window_counts.get_mut(&right_char) {
                *wc_count += 1;
            } else {
                window_counts.insert(right_char, 1);
            }

            if t_counts.contains_key(&right_char) {
                if t_counts.get(&right_char) == window_counts.get(&right_char) {
                    formed += 1;
                }
            }

            while left_ptr <= right_ptr && formed == t_counts.len() {
                let len = (right_ptr - left_ptr + 1) as isize;
                let left_char = s_bytes[left_ptr];

                if result.0 == -1 || len < result.0 {
                    result = (len, left_ptr, right_ptr);
                }

                if let Some(wc_count) = window_counts.get_mut(&left_char) {
                    *wc_count -= 1;
                }

                if t_counts.contains_key(&left_char) {
                    if t_counts.get(&left_char) > window_counts.get(&left_char) {
                        formed -= 1;
                    }
                }

                left_ptr += 1;
            }

            right_ptr += 1;
        }

        if result.0 != -1 {
            let utf8s = s_bytes[result.1..(result.2 + 1)].to_vec();
            String::from_utf8(utf8s).unwrap_or("".to_string())
        } else {
            String::new()
        }
    }
}

fn main() {
    assert_eq!("BANC".to_string(), Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()));
    assert_eq!("a".to_string(), Solution::min_window("a".to_string(), "a".to_string()));
    assert_eq!("a".to_string(), Solution::min_window("aaaaaaaa".to_string(), "a".to_string()));
    assert_eq!("".to_string(), Solution::min_window("a".to_string(), "aaa".to_string()));
}
