use Q7_reverse_integer::Solution;

fn main() {
    assert_eq!(321, Solution::reverse(123));
    assert_eq!(21, Solution::reverse(120));
    assert_eq!(-123, Solution::reverse(-321));
}