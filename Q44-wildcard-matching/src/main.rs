use std::collections::HashMap;

struct Solution {}

impl Solution {

    fn remove_duplicate_stars(s: String) -> String {
        let mut chrs: Vec<u8> = vec![];

        for chr in s.into_bytes() {
            if let Some(last) = chrs.last() {
                if last == &b'*' && chr == b'*' {
                    continue;
                } else {
                    chrs.push(chr);
                }
            } else {
                chrs.push(chr);
            }
        }

        String::from_utf8(chrs).unwrap()
    }

    fn helper<'a>(s: &'a [u8], p: &'a [u8], dp: &mut HashMap<(&'a [u8], &'a [u8]), bool>) -> bool {
        let v;

        if dp.contains_key(&(s, p)) {
            return *dp.get(&(s, p)).unwrap();
        }

        if s == p ||  p == b"*" {
            v = true;
        } else if s.is_empty() || p.is_empty() {
            v = false;
        } else if s[0] == p[0] || p[0] == b'?' {
            v = Solution::helper(&s[1..], &p[1..], dp);
        } else if p[0] == b'*' {
            v = Solution::helper(s, &p[1..], dp) || Solution::helper(&s[1..], p, dp);
        } else {
            v = false;
        }

        dp.insert((s, p), v);
        *dp.get(&(s, p)).unwrap()
    }


    pub fn is_match(s: String, mut p: String) -> bool {
        let mut dp = HashMap::new();

        p = Solution::remove_duplicate_stars(p);
        Solution::helper(s.as_bytes(), p.as_bytes(), &mut dp)
    }
}

struct Solution2 {}

impl Solution2 {

    #[inline]
    fn fill(mut v: &mut Vec<Vec<bool>>, m: usize, n: usize) {
        for i in 0..m {
            let mut cols = vec![];

            for j in 0..n {
                cols.push(false);
            }

            v.push(cols);
        }
    }

    pub fn is_match(s: String, p: String) -> bool {
        if p == s || p == "*".to_string() {
            true
        } else if p.is_empty() || s.is_empty() {
            false
        } else {
            let mut dp = vec![];
            let p_bytes = p.into_bytes();
            let s_bytes = s.into_bytes();
            let s_len = s_bytes.len();
            let p_len = p_bytes.len();

            Solution2::fill(&mut dp, s_len + 1, p_len + 1);
            dp[0][0] = true;


            for p_idx in 1..=p_len {
                for s_idx in 1..=s_len {
                    // notice, if p_bytes[p_idx - 1] is *,
                    // it means that dp[_s_idx > __s_idx][p_idx - 1] which _s_idx is the index of the first true element (dp[__s_idx][p_idx - 1] == true)
                    // should be set as true, because * can match string with arbitrary length
                    if p_bytes[p_idx - 1] == b'*' {
                        for _s_idx in 0..=s_idx {
                            if dp[_s_idx][p_idx - 1] {
                                for start in _s_idx..=s_idx {
                                    dp[start][p_idx] = true;
                                }
                                break;
                            }
                        }
                    } else if p_bytes[p_idx - 1] == b'?' || p_bytes[p_idx - 1] == s_bytes[s_idx - 1] {
                        dp[s_idx][p_idx] = dp[s_idx - 1][p_idx - 1];
                    } else {
                        dp[s_idx][p_idx] = false;
                    }
                }
            }

            // for p_idx in 1..=p_len {
            //     if p_bytes[p_idx - 1] == b'*' {
            //         let mut s_idx = 1;
            //
            //         while !dp[s_idx - 1][p_idx - 1] && s_idx <= s_len {
            //             s_idx += 1;
            //         }
            //
            //         dp[s_idx - 1][p_idx] = dp[s_idx - 1][p_idx - 1];
            //
            //         while s_idx <= s_len {
            //             dp[s_idx][p_idx] = true;
            //             s_idx += 1;
            //         }
            //     } else if p_bytes[p_idx - 1] == b'?' {
            //         for s_idx in 1..=s_len {
            //             dp[s_idx][p_idx] = dp[s_idx - 1][p_idx - 1];
            //         }
            //     } else {
            //         for s_idx in 1..=s_len {
            //             dp[s_idx][p_idx] = dp[s_idx - 1][p_idx - 1] && p_bytes[p_idx - 1] == s_bytes[s_idx - 1]
            //         }
            //     }
            // }

            dp[s_len][p_len]
        }
    }
}

fn main() {
    assert_eq!(true, Solution2::is_match("aa".to_string(), "a*".to_string()));
    assert_eq!(true, Solution2::is_match("aa".to_string(), "*".to_string()));
    assert_eq!(false, Solution2::is_match("cb".to_string(), "?a".to_string()));
    assert_eq!(true, Solution2::is_match("adceb".to_string(), "*a*b".to_string()));
    assert_eq!(false, Solution2::is_match("acdcb".to_string(), "a*c?b".to_string()));
    assert_eq!(true, Solution2::is_match("".to_string(), "".to_string()));
    assert_eq!(true, Solution2::is_match("ho".to_string(), "ho**".to_string()));
    assert_eq!(true, Solution2::is_match("abc".to_string(), "a*b*c*".to_string()));
    assert_eq!(true, Solution2::is_match("aaaabaaaabbbbaabbbaabbaababbabbaaaababaaabbbbbbaabbbabababbaaabaabaaaaaabbaabbbbaababbababaabbbaababbbba".to_string(), "*****b*aba***babaa*bbaba***a*aaba*b*aa**a*b**ba***a*a*".to_string()));

    assert_eq!(false, Solution2::is_match(
        "abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_string(),
        "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb".to_string()));
    assert_eq!(false, Solution2::is_match(
         "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        "*aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa*".to_string(),
    ));
}