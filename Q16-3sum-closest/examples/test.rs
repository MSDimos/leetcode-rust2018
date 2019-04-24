use Q16_3sum_closest::Solution;

fn main() {
    assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    assert_eq!(0, Solution::three_sum_closest(vec![0, 2, 1, -3], 1));

}