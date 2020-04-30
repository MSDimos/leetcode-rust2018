use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group: HashMap<String, Vec<String>> = HashMap::new();
        strs.sort();

        for str in strs {
            let mut bytes = str.clone().into_bytes();
            bytes.sort();
            let key = String::from_utf8(bytes).unwrap();

            if group.contains_key(&key) {
                if let Some(v) = group.get_mut(&key) {
                    v.push(str);
                }
            } else {
                group.insert(key, vec![str]);
            }
        }

        let mut result = vec![];

        for value in group.values() {
            result.push(value.clone());
        }

        result.sort_by(|a, b| b.len().cmp(&a.len()));
        result
    }
}

fn main() {
    let a: Vec<Vec<String>> = vec![
            vec!["ate", "eat", "tea"],
            vec!["nat", "tan"],
            vec!["bat"]
        ].into_iter()
        .map(|v| {
            let strings: Vec<String> = v.into_iter().map(|s| s.to_string()).collect();
            strings
        }).collect();
    let b: Vec<Vec<String>> = Solution::group_anagrams(
    vec!["eat", "tea", "tan", "ate", "nat", "bat"].into_iter()
        .map(|s| s.to_string())
        .collect()
    );
    assert_eq!(a, b);
}
