pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            false
        } else {
            let mut rev = 0;
            while x > rev {
                rev = rev * 10 + x % 10;
                x /= 10;
            }

            if x == rev || ( x < rev && (rev / 10) == x ) {
                true
            } else {
                false
            }
        }
    }
}
