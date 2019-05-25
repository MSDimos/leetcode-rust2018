//use std::collections::HashMap;
//
//pub struct Solution;
//
// 常规解法，用一个长度为word_width * words_num的窗来不断右移
// 每次计算是否匹配
// 效率很低,大概168ms左右，但还是能过
//impl Solution {
//    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
//
//        if words.is_empty() {
//            return vec![];
//        }
//
//        let mut r = vec![];
//        let words_num = words.len();
//        let word_width = words[0].len();
//        let window_width = word_width * words_num;
//        let words_map = {
//            let mut m = HashMap::new();
//
//            for word in words {
//                if !m.contains_key(&word) {
//                    m.insert(word, 1);
//                } else {
//                    let num = m.get(&word).unwrap().clone() + 1;
//                    m.insert(word, num);
//                }
//            }
//
//            m
//        };
//
//        if s.len() < window_width {
//            return vec![];
//        }
//
//        for i in 0..=(s.len() - window_width) {
//            let sub_str = &s[i..(i + window_width)];
//            let mut map = words_map.clone();
//            let mut matched = true;
//
//
//            for j in 0..words_num {
//                let word = &sub_str[j * word_width..(j + 1) * word_width];
//
//                if map.contains_key(word) {
//                    if let Some(num) = map.get(word) {
//                        if num > &0 {
//                            *map.get_mut(word).unwrap() -= 1;
//                        } else {
//                            matched = false;
//                            break;
//                        }
//                    }
//                } else {
//                    matched = false;
//                    break;
//                }
//            }
//
//            if matched {
//                r.push(i as i32);
//            }
//        }
//
//        r
//    }
//}

// 滑动窗口
// 效率大概8ms左右
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut r = vec![];

        if s.is_empty() || words.is_empty() {
            return r;
        }

        let word_width = words[0].len();
        let words_num = words.len();
        let window_width = word_width * words_num;
        let mut words_map = HashMap::new();

        for word in words {
            let prev: i32 = *words_map.get(&word).unwrap_or(&0);
            words_map.insert(word, prev + 1);
        }

        // 从0..word_width循环即可循环可能的所有单词数
        // 这是因为我们的要求是连续的
        // 并且单词的长度都是固定的，均为word_width
        // 因此随意选取s中一部分连续的长度为word_width的字符串
        // 他们都只能是两种情况：是一个完整的单词，是两个单词的分别前缀后缀
        for i in 0..word_width {
            let mut left = i;
            let mut right = i;
            let mut count = 0;
            let mut temp_map = HashMap::new();

            // 不断增加left-right的长度
            // 匹配其中所有的单词
            // 如果发现其中某一个单词的出现次数与给定的次数不匹配(多了)
            // 则把left前移缩小left-right的范围
            // 如果left-right范围内的单词数与给定的相同，并且出现次数也匹配
            // 则说明该区间满足条件
            while right + word_width <= s.len() {
                let w = &s[right..(right + word_width)];
                let prev: i32 = *temp_map.get(w).unwrap_or(&0);

                right += word_width;

                // left-right区间的某个单词是不存在于words中的
                // 说明任何包含这个单词的区间都是不满足条件的
                // 因此把left置位该不匹配单词的末尾后，开始下一轮循环
                // 减少不必要的循环
                if !words_map.contains_key(w) {
                    count = 0;
                    left = right;
                    temp_map.clear();
                } else {
                    // 该单词存在于words中
                    // 更新temp_map中出现的单词的情况，把这个单词出现的次数加1
                    temp_map.insert(w, prev + 1);
                    count += 1;

                    // 当前区间的末尾单词在区间的出现次数已经超过了所给的words中出现的次数
                    // 所限left-right的范围，直到出现的次数相符合
                    // left <= right保证了count不会溢出
                    while (temp_map.get(w).unwrap_or(&0) > words_map.get(w).unwrap_or(&0)) && (left <= right) {
                        let temp_word = &s[left..left + word_width];

                        count -= 1;
                        temp_map.insert(temp_word, *temp_map.get(temp_word).unwrap_or(&0) - 1);
                        left += word_width;
                    }

                    // count统计的是已经满足匹配的单词的个数
                    // 如果恰好等于words中单词的个数
                    // 那么说明当前的left-right区间满足条件
                    if count == words_num {
                        r.push(left as i32);
                    }
                }
            }
        }

        r
    }
}