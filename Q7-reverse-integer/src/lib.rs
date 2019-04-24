pub struct Solution;
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev = 0;
        let pos_piece = i32::max_value() / 10;
        let neg_piece = i32::min_value() / 10;
        let mut pop = 0;

        while x != 0 {
            pop = x % 10;
            x /= 10;
            if rev > pos_piece || (rev == pos_piece && pop > 7) {
                return 0;
            } else if rev < neg_piece || (rev == neg_piece && pop < -8) {
                return 0;
            } else {
                rev = rev * 10 + pop;
            }
        }

        return rev;

        rev
    }
}