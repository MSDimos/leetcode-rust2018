use std::collections::HashSet;

struct Solution {}

impl Solution {

    // backtracking
    fn helper(
        result: &mut Vec<Vec<i32>>, sample: &mut Vec<i32>,
        pool: &mut HashSet<i32>, nums: &Vec<i32>
    ) {
        for num in nums {
            if !pool.contains(num) {
                sample.push(*num);
                pool.insert(*num);

                if sample.len() == nums.len() {
                    // IDEA bug, add type converting syntax to solve it
                    result.push(sample.clone() as Vec<i32>);
                } else {
                    Solution::helper(result, sample, pool, nums);
                }

                pool.remove(num);
                sample.pop();
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut sample = vec![];
        let mut pool = HashSet::new();

        Solution::helper(&mut result, &mut sample, &mut pool, &nums);
        result
    }
}


fn main() {
    // let mut a = vec![vec![]];
    // let mut b = vec![1, 2, 3, 4];
    // let ptr = &mut b;
    //
    // a.push(ptr.clone());
    // eprintln!("{:#?}", ptr);

    assert_eq!(
        vec![
            vec![1,2,3],
            vec![1,3,2],
            vec![2,1,3],
            vec![2,3,1],
            vec![3,1,2],
            vec![3,2,1]
        ],
        Solution::permute(vec![1, 2, 3])
    );
}
