pub struct Solution;

// 根据中心展开，尽可能地查找最长回文穿
pub fn expand_around_center(s: &Vec<u8>, center_left: usize, center_right: usize) -> (usize, usize) {
    let mut l = center_left;
    let mut r = center_right;
    let len = s.len() - 1;

    while l > 0 && r < len && l < len && r > 0 {
        if s.get(l - 1) == s.get(r + 1) {
            l -= 1;
            r += 1;
        } else {
            break;
        }
    }

    (l ,r)
}

impl Solution {
    pub fn longest_palindrome(mut s: String) -> String {
        let mut bytes = s.into_bytes();
        let len = bytes.len();
        let mut range: Option<(usize, usize)> = None;
        let mut range_len = 0;

        for i in 0..len {
            // 奇数长度的回文
            let r_odd = expand_around_center(&bytes, i, i);
            // 偶数长度的回文
            let r_even = if bytes.get(i) == bytes.get(i + 1) {
                expand_around_center(&bytes, i, i + 1)
            } else {
                (i, i)
            };

            if (r_odd.1 - r_odd.0) >= range_len {
                range = Some(r_odd);
                range_len = r_odd.1 - r_odd.0;
            }

            if (r_even.1 - r_even.0) >= range_len {
                range = Some(r_even);
                range_len = r_even.1 - r_even.0;
            }
        }

        if let Some((l, r)) = range {
            String::from_utf8(bytes.drain(l..r + 1).collect()).unwrap_or(String::new())
        } else {
            String::new()
        }
    }
}