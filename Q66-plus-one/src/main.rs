struct Solution {}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        for digit in digits.iter_mut().rev() {
            let temp = *digit + carry;

            *digit = temp % 10;
            carry = temp / 10;
        }

        if carry != 0 {
            digits.insert(0, carry);
        }

        digits
    }
}

fn main() {
    assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    assert_eq!(vec![1, 0, 0, 0], Solution::plus_one(vec![9, 9, 9]));

}
