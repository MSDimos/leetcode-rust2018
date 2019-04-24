pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let mut common_str = if !strs.is_empty() {
            strs.remove(0).into_bytes()
        } else {
            vec![]
        };
        let mut end_idx = common_str.len();

        for s in strs.into_iter() {
            let bytes = s.bytes().into_iter();
            let mut idx = 0;

            if bytes.len() > 0 {
                for (i, val) in bytes.enumerate() {
                    if Some(&val) == common_str.get(i) {
                        idx += 1;
                    } else {
                        break;
                    }
                }

                if end_idx >= idx {
                    end_idx = idx;
                }
            } else {
                return String::new();
            }
        }

        String::from_utf8(common_str.drain(0..end_idx).collect()).unwrap_or(String::new())
    }
}