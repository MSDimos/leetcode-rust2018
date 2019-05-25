//pub struct Solution;
//
//impl Solution {
//    pub fn longest_valid_parentheses(s: String) -> i32 {
//        // 这个题直接用动态规划更好一点儿
//        let mut dp = vec![0; s.len()];
//        let mut s = s.chars().collect::<Vec<char>>();
//        let mut max = 0;
//
//        for i in 0..s.len() {
//            if i > 0 && s[i] == ')' {
//                if s[i - 1] == '(' {
//                    if i >= 2 {
//                        dp[i] = dp[i - 2] + 2;
//                    } else {
//                        dp[i] = 2;
//                    }
//                    // s[i - dp[i - 1] - 1] == '('如果成立，那么s[i - dp[i - 1] - 1]一定为0
//                    // 因此i-dp[i - 1] - 1实际上是一个断层
//                    // 如果此时i为），那么刚好可能与它匹配
//                    // 所以才是dp[i] = dp[i - 1] + 2 + dp[i - dp[i - 1] - 2]
//                } else if s[i - 1] == ')' && i - dp[i - 1] >= 1 && s[i - dp[i - 1] - 1] == '(' {
//                    if i - dp[i - 1] >= 2 {
//                        dp[i] = dp[i - 1] + 2 + dp[i - dp[i - 1] - 2];
//                    } else {
//                        dp[i] = dp[i - 1] + 2;
//                    }
//                }
//
//                if dp[i] > max {
//                    max = dp[i];
//                }
//            }
//        }
//
//        max as i32
//    }
//}
//

// 方法二
// 找到所有的()嵌套组合，例如(()), ((()))之类的
// 如果发现某两个组合刚好临近，那么把他们合并成一个新的组合，例如()(), (())()之类的
// 依次类推，找到所有有效组合的最大长度
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {

        if s.len() < 2 {
            return 0;
        }

        let mut l = 0;
        let mut r = 0;
        let mut max = 0;
        let mut i = 0;
        let mut size: HashMap<usize, usize> = HashMap::new();
        let chars = s.clone().chars().collect::<Vec<char>>();

        while i < s.len() - 1 {
            if chars[i] == '(' && chars[i + 1] == ')' {
                l = i;
                r = i + 1;

                loop {
                    if r < chars.len() - 1 && l > 0 && chars[l - 1] == '(' && chars[r + 1] == ')' {
                        l -= 1;
                        r += 1;
                    } else {
                        if l > 0 && size.contains_key(&(l - 1)) {
                            let pl = *size.get(&(l - 1)).unwrap();

                            l = pl;
                        } else {
                            size.insert(r, l);
                            i = r;
                            break;
                        }
                    }
                }

                if r - l + 1 > max {
                    max = r - l + 1;
                }
            }

            i += 1;
        }

        max as i32
    }
}