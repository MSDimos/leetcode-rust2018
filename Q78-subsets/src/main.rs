struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets = vec![vec![]];
        let mut subset = vec![];

        for i in 1..=nums.len() {
            Solution::helper(0, (nums.len(), i), &nums, &mut subset, &mut subsets);
        }

        subsets
    }

    fn helper(start: usize, (n, k): (usize, usize), nums: &Vec<i32>, subset: &mut Vec<i32>, subsets: &mut Vec<Vec<i32>>) {
        if subset.len() == k {
            subsets.push(subset.clone() as Vec<i32>);
        } else {
            if n >= start && n - start + 1 >= k - subset.len() {
                for i in start..n {
                    subset.push(nums[i]);
                    Solution::helper(i + 1, (n, k), nums, subset, subsets);
                    subset.pop();
                }
            }
        }
    }
}

fn main() {
    let output = vec![
        vec![],
        vec![1],
        vec![2],
        vec![3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3]
    ];
    assert_eq!(output, Solution::subsets(vec![1, 2, 3]));
}
