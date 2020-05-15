use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut min_len = 0;

        if !word_list.contains(&end_word) {
            0
        } else {
            Self::bfs(begin_word, end_word, word_list, &mut min_len);
            min_len
        }
    }

    fn bfs(begin_word: String, end_word: String, word_list: Vec<String>, ans: &mut i32) {
        let mut queue = VecDeque::new();
        let mut path = vec![];
        let mut visited = HashSet::new();
        let dict = HashSet::from_iter(word_list.into_iter());

        path.push(begin_word.clone());
        visited.insert(begin_word);
        queue.push_back(path);

        while !queue.is_empty() {
            let size = queue.len();
            let mut sub_visited = HashSet::new();

            for _ in 0..size {
                let mut p = queue.pop_front().unwrap();
                let neighbors = Solution::get_neighbors(&p[p.len() - 1], &dict);

                for neighbor in neighbors.into_iter() {
                    if !visited.contains(&neighbor) {
                        if neighbor == end_word {
                            *ans = p.len() as i32 + 1;
                            return;
                        }

                        p.push(neighbor.clone());
                        queue.push_back(p.clone());
                        p.pop();
                        sub_visited.insert(neighbor);
                    }
                }
            }

            for visited_str in sub_visited {
                visited.insert(visited_str);
            }
        }
    }

    fn get_neighbors(node: &str, dict: &HashSet<String>) -> Vec<String> {
        let mut res = vec![];
        let mut chrs = node.to_string().into_bytes();

        for chr in b'a'..=b'z' {
            for i in 0..chrs.len() {
                if chrs[i] == chr {
                    continue;
                } else {
                    let old_chr = chrs[i];
                    chrs[i] = chr;
                    let new_str = String::from_utf8(chrs.clone()).unwrap_or_default();

                    if dict.contains(&new_str) {
                        res.push(new_str);
                    }

                    chrs[i] = old_chr;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list: Vec<String> = vec!["hot", "dot", "dog", "lot", "log", "cog"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let answer = vec![
            ["hit", "hot", "dot", "dog", "cog"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            ["hit", "hot", "lot", "log", "cog"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        ];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);

        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let answer: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);

        let begin_word = "hot".to_string();
        let end_word = "dog".to_string();
        let word_list = vec!["hot", "dog"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let answer: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
    }
}
