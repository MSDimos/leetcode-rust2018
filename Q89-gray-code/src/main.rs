struct Solution {}

impl Solution {
    // https://en.wikipedia.org/wiki/Gray_code
    pub fn gray_code(n: i32) -> Vec<i32> {
        let total_number = 1 << n;
        let mut gray_codes = vec![];

        for i in 0..total_number {
            gray_codes.push(i ^ (i >> 1));
        }

        gray_codes
    }
}

fn main() {
    assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2));
}
