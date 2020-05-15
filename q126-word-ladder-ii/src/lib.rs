use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    fn is_one_diff(begin_word: &str, cur_word: &str) -> bool {
        let begin_word = begin_word.as_bytes();
        let cur_word = cur_word.as_bytes();
        let mut count = 0;

        for i in 0..begin_word.len() {
            if begin_word[i] != cur_word[i] {
                count += 1;
            }

            if count >= 2 {
                return false;
            }
        }

        count == 1
    }

    fn find_ladder_helper_s1(
        begin_wrod: &str,
        end_word: &str,
        word_list: &[String],
        temp: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
        min: &mut usize,
    ) {
        if begin_wrod == end_word {
            match (*min).cmp(&temp.len()) {
                Ordering::Greater => {
                    ans.clear();
                    *min = temp.len();
                    ans.push(temp.clone() as Vec<String>);
                }
                Ordering::Equal => {
                    ans.push(temp.clone() as Vec<String>);
                }
                _ => return,
            }
            return;
        }

        if temp.len() >= *min {
            return;
        }

        for i in 0..word_list.len() {
            let cur_word = &word_list[i];

            if temp.contains(cur_word) {
                continue;
            }

            if Solution::is_one_diff(begin_wrod, cur_word) {
                temp.push(cur_word.clone());
                Solution::find_ladder_helper_s1(cur_word, end_word, word_list, temp, ans, min);
                temp.pop();
            }
        }
    }

    pub fn find_ladders_s1(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut ans = vec![];
        let mut temp = vec![];
        let mut min = usize::max_value();

        temp.push(begin_word.clone());
        Solution::find_ladder_helper_s1(
            &begin_word,
            &end_word,
            &word_list,
            &mut temp,
            &mut ans,
            &mut min,
        );
        ans
    }

    fn bfs_s2(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        let mut queue = VecDeque::new();
        let mut path = vec![];
        let mut is_found = false;
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
                            is_found = true;
                            p.push(neighbor.clone());
                            ans.push(p.clone());
                            p.pop();
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

            if is_found {
                break;
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

    pub fn find_ladders_s2(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut ans = vec![];

        if !word_list.contains(&end_word) {
            ans
        } else {
            Self::bfs_s2(begin_word, end_word, word_list, &mut ans);
            ans
        }
    }

    fn bfs_s3(
        begin_word: String,
        end_word: String,
        mut word_set: HashSet<String>,
        map: &mut HashMap<String, Vec<String>>,
    ) {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();

        set1.insert(begin_word);
        set2.insert(end_word);
        Self::bfs_helper(&set1, &set2, &mut word_set, true, map);
    }

    fn bfs_helper(
        set1: &HashSet<String>,
        set2: &HashSet<String>,
        word_set: &mut HashSet<String>,
        direction: bool,
        map: &mut HashMap<String, Vec<String>>,
    ) -> bool {
        if set1.is_empty() {
            return false;
        }

        if set1.len() > set2.len() {
            return Solution::bfs_helper(set2, set1, word_set, !direction, map);
        }

        {
            for str in set1.iter() {
                word_set.remove(str);
            }

            for str in set2.iter() {
                word_set.remove(str);
            }
        }

        let mut done = false;
        let mut set = HashSet::new();

        for str in set1.iter() {
            let mut chars = str.to_string().into_bytes();

            for i in 0..chars.len() {
                for ch in b'a'..=b'z' {
                    if chars[i] == ch {
                        continue;
                    } else {
                        let old_ch = chars[i];
                        chars[i] = ch;
                        let word = String::from_utf8(chars.clone()).unwrap();
                        chars[i] = old_ch;

                        let (key, val);

                        if direction {
                            key = str.to_string();
                            val = word.clone();
                        } else {
                            key = word.clone();
                            val = str.to_string();
                        };

                        let mut list = map.remove(&key).unwrap_or_default();

                        if set2.contains(&word) {
                            done = true;
                            list.push(val.clone());
                        }

                        if !done && word_set.contains(&word) {
                            set.insert(word);
                            list.push(val);
                        }

                        if !list.is_empty() {
                            map.insert(key, list);
                        }
                    }
                }
            }
        }

        done || Solution::bfs_helper(set2, &set, word_set, !direction, map)
    }

    fn find_ladders_helper_s3(
        begin_word: &str,
        end_word: &str,
        map: &HashMap<String, Vec<String>>,
        temp: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if begin_word == end_word {
            ans.push(temp.clone() as Vec<String>);
        }

        if let Some(neighbors) = map.get(begin_word) {
            for neighbor in neighbors.iter() {
                temp.push(neighbor.clone());
                Self::find_ladders_helper_s3(neighbor, end_word, map, temp, ans);
                temp.pop();
            }
        }
    }

    pub fn find_ladders_s3(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut ans = vec![];
        let mut map = HashMap::new();
        let mut temp = vec![];
        let word_set = HashSet::from_iter(word_list.into_iter());

        if !word_set.contains(&end_word) {
            return ans;
        }

        Self::bfs_s3(begin_word.clone(), end_word.clone(), word_set, &mut map);
        temp.push(begin_word.clone());
        Self::find_ladders_helper_s3(&begin_word, &end_word, &map, &mut temp, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works_s1() {
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
        assert_eq!(
            Solution::find_ladders_s1(begin_word, end_word, word_list),
            answer
        );

        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let answer: Vec<Vec<String>> = vec![];
        assert_eq!(
            Solution::find_ladders_s1(begin_word, end_word, word_list),
            answer
        );
    }

    #[test]
    fn it_works_s2() {
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
        assert_eq!(
            Solution::find_ladders_s2(begin_word, end_word, word_list),
            answer
        );

        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let answer: Vec<Vec<String>> = vec![];
        assert_eq!(
            Solution::find_ladders_s2(begin_word, end_word, word_list),
            answer
        );
    }

    #[test]
    fn it_works_s3() {
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
        assert_eq!(
            Solution::find_ladders_s3(begin_word, end_word, word_list),
            answer
        );

        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vec!["hot", "dot", "dog", "lot", "log"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let answer: Vec<Vec<String>> = vec![];
        assert_eq!(
            Solution::find_ladders_s3(begin_word, end_word, word_list),
            answer
        );
    }
}
