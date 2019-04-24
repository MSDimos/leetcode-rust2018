use Q18_4sum::Solution;

fn main() {
    let res = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
    assert_eq!(vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]], res);

    let res = Solution::four_sum(vec![0, 0, 0, 0], 0);
    assert_eq!(vec![vec![0, 0, 0, 0]], res);
}