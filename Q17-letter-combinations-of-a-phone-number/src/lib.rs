// 键盘映射
const BUTTONS: [[char; 4]; 8] = [
    // '-'占位用
    ['a', 'b', 'c', '-'],
    ['d', 'e', 'f', '-'],
    ['g', 'h', 'i', '-'],
    ['j', 'k', 'l', '-'],
    ['m', 'n', 'o', '-'],
    ['p', 'q', 'r', 's'],
    ['t', 'u', 'v', '-'],
    ['w', 'x', 'y', 'z'],
];


pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {

        if digits.len() == 0 {
            return vec![];
        }

        // 把字符转换为数字, 2 对应的是转换成0， 9转换成7
        let nums: Vec<usize> = digits.bytes().map(|x| (x - 50) as usize).collect();
        let mut s = String::new();
        let mut result = vec![];

        Solution::dfs(&mut s, 0, &nums, &mut result);

        result

    }

    fn dfs(s: &mut String, idx: usize, digits: &Vec<usize>, result: &mut Vec<String>) {
        if idx < digits.len() {
            for chr in BUTTONS[digits[idx]].iter() {
                if chr != &'-' {
                    s.push(chr.clone());
                    Solution::dfs(s, idx + 1, digits, result);
                    s.remove(s.len() - 1);
                }
            }
        } else {
            result.push(s.clone());
        }
    }
}