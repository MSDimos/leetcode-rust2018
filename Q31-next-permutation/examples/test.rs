use Q31_next_permutation::Solution;

fn main() {
    let mut v = vec![1, 3, 2];

    Solution::next_permutation(&mut v);

    println!("{:#?}", v);
}