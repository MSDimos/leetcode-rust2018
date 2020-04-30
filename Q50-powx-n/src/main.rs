struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut m = if n < 0 { (-n - 1) as usize } else { n as usize };
        let mut res = 1.0;
        let mut basic = x;

        while m > 0 {
            if m & 0x0001 == 0x0001 {
                res *= basic;
            }

            m >>= 1;
            basic *= basic;
        }

        if n < 0 { 1.0 / (res * x) } else { res }
    }
}

fn main() {
}
