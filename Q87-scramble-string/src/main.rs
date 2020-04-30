use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        // solution 1
        // let s1 = s1.into_bytes();
        // let s2 = s2.into_bytes();
        // let mut matched = HashMap::new();
        //
        // Solution::helper(&s1[..], &s2[..], &mut matched)

        // solution 2
        if s1.len() != s2.len() {
            return false;
        }

        if s1 == s2 {
            return true;
        }

        let s1_bytes = s1.into_bytes();
        let s2_bytes = s2.into_bytes();
        let len = s1_bytes.len();
        let mut letters: [i32; 26] = [0; 26];
        let mut dp = vec![vec![vec![false; len]; len]; len + 1];

        for i in 0..s1_bytes.len() {
            letters[(s1_bytes[i] - b'a') as usize] += 1;
            letters[(s2_bytes[i] - b'a') as usize] -= 1;
        }

        for count in letters.iter() {
            if count != &0 {
                return false;
            }
        }

        for l in 0..=len {
            for i in 0..=(len - l) {
                for j in 0..=(len - l) {
                    if l == 1 {
                        dp[l][i][j] = s1_bytes[i] == s2_bytes[j];
                    } else {
                        for q in 1..l {
                            dp[l][i][j] = (dp[q][i][j] && dp[l - q][i + q][j + q]) || (dp[q][i][j + l - q] && dp[l - q][i + q][j]);

                            // if matched already, skip all next q
                            // because next q maybe cause situation that doesn't match
                            // so it will be false and overwrite dp[l][i][j] as false even it's true already
                            if dp[l][i][j] {
                                break;
                            }
                        }
                    }
                }
            }
        }

        dp[len][0][0]
    }

    fn helper(s1: &[u8], s2: &[u8], matched: &mut HashMap<String, bool>) -> bool {
        let key = format!("{:?}-{:?}", s1, s2);

        if matched.contains_key(&key) {
            return *matched.get(&key).unwrap();
        }

        if s1.len() != s2.len() {
            matched.insert(key, false);
            return false;
        }

        if s1 == s2 {
            matched.insert(key, true);
            return true;
        }

        let mut letters: [i32; 26] = [0; 26];

        for i in 0..s1.len() {
            letters[(s1[i] - b'a') as usize] += 1;
            letters[(s2[i] - b'a') as usize] -= 1;
        }

        for count in letters.iter() {
            if count != &0 {
                matched.insert(key, false);
                return false;
            }
        }

        for i in 1..s1.len() {
            // do not swap two children
            if Solution::helper(&s1[0..i], &s2[0..i], matched) &&
                Solution::helper(&s1[i..], &s2[i..], matched) {
                matched.insert(key, true);
                return true;
            }

            // swap two children
            if Solution::helper(&s1[i..], &s2[0..s2.len() - i], matched) &&
                Solution::helper(&s1[0..i], &s2[(s2.len() - i)..], matched) {
                matched.insert(key, true);
                return true;
            }
        }

        matched.insert(key, false);
        false
    }
}

fn main() {
    assert_eq!(true, Solution::is_scramble("great".to_string(), "rgeat".to_string()));
    assert_eq!(false, Solution::is_scramble("abcde".to_string(), "caebd".to_string()));
}
