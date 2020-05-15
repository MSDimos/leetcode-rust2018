use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut lens: Vec<usize> = word_dict.iter().map(|s| s.len()).collect();
        let mut set = HashSet::new();
        let mut mem = HashMap::new();

        lens.sort();
        lens.dedup_by(|a, b| (*a).eq(b));

        for word in word_dict {
            set.insert(word.into_bytes());
        }

        Self::helper(s.as_bytes(), &lens, &set, &mut mem)
    }

    fn helper<'b>(
        bytes: &'b [u8],
        lens: &[usize],
        word_dict: &HashSet<Vec<u8>>,
        mem: &mut HashMap<&'b [u8], Vec<String>>,
    ) -> Vec<String> {
        if mem.contains_key(bytes) {
            return mem[bytes].clone();
        }

        if bytes.is_empty() {
            return vec![String::new()];
        }

        let mut res = vec![];

        for len in lens {
            if bytes.len() < *len {
                break;
            }

            if word_dict.contains(&bytes[..*len]) {
                let tmp = Self::helper(&bytes[*len..], lens, word_dict, mem);
                let word = String::from_utf8((&bytes[..*len]).to_vec()).unwrap();

                for tmp_s in tmp {
                    if tmp_s.is_empty() {
                        res.push(word.clone());
                    } else {
                        res.push((word.clone() + " ") + &tmp_s);
                    }
                }
            }
        }

        mem.insert(bytes, res.clone());
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let s = "catsanddog".to_string();
        let word_dict = vec![
            "cat".to_string(),
            "cats".to_string(),
            "and".to_string(),
            "sand".to_string(),
            "dog".to_string(),
        ];
        assert_eq!(
            Solution::word_break(s, word_dict),
            vec!["cat sand dog".to_string(), "cats and dog".to_string()]
        );

        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
        let word_dict: Vec<String> = vec![
            "a",
            "aa",
            "aaa",
            "aaaa",
            "aaaaa",
            "aaaaaa",
            "aaaaaaa",
            "aaaaaaaa",
            "aaaaaaaaa",
            "aaaaaaaaaa",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let temp: Vec<String> = vec![];
        assert_eq!(Solution::word_break(s, word_dict), temp);
    }
}
