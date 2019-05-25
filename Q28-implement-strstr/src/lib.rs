pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let p = haystack.find(&needle);

        if let Some(i) = p {
            i as i32
        } else {
            -1
        }
    }
}