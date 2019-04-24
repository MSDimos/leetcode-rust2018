#![feature(int_error_matching)]
pub struct Solution;

// leetcode 不支持nightly，这段代码无法使用
impl Solution {
    pub fn my_atoi(mut s: String) -> i32 {
        let chars = s.chars();
        let mut left = 0;
        let mut right = 0;
        let mut whitespaces = true;

        for (idx, chr) in chars.enumerate() {
            if chr.is_whitespace() {
                if whitespaces {
                    continue;
                } else {
                    break;
                }
            } else {
                whitespaces = false;
                if chr.is_numeric() {
                    right = idx + 1;
                } else if chr == '+' || chr == '-' {
                    if left == right {
                        left = idx;
                        right = idx;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        let result = s.drain(left..right).collect::<String>().parse();

        if let Ok(num) = result {
            num
        } else {
            match result {
                Ok(num) => num,
                Err(error) => {
                    match error.kind() {
                        &core::num::IntErrorKind::Underflow => {
                            -2147483648
                        },
                        &core::num::IntErrorKind::Overflow => {
                            2147483647
                        },
                        _ => 0,
                    }
                }
            }
        }
    }
}