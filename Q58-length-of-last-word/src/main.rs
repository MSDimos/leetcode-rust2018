
struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut s_bytes = s.into_bytes();
        let mut len = 0;

        // trim end
        while !s_bytes.is_empty() && s_bytes[s_bytes.len() - 1] == b' ' {
            s_bytes.pop();
        }

        for byte in s_bytes.into_iter().rev() {
            if byte == b' ' {
                break;
            }

            len += 1;
        }

        len
    }
}


fn main() {
    assert_eq!(5, Solution::length_of_last_word("Hello World".to_string()));
    assert_eq!(1, Solution::length_of_last_word("a ".to_string()));
    assert_eq!(0, Solution::length_of_last_word("   ".to_string()));
}
