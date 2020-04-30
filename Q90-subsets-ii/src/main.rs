use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // solution 1
        // let mut subsets = vec![vec![]];
        // let mut num_count: HashMap<i32, usize> = HashMap::new();
        //
        // for num in &nums {
        //     if let Some(count) = num_count.get_mut(num) {
        //         *count += 1;
        //     } else {
        //         num_count.insert(*num, 1);
        //     }
        // }
        //
        // for (num, count) in num_count {
        //     let _subsets = subsets.clone();
        //
        //     for mut subset in _subsets {
        //         for i in 1..=count {
        //             let mut repeats = vec![num; i];
        //             let mut s = subset.clone();
        //             s.append(&mut repeats);
        //             subsets.push(s);
        //         }
        //     }
        // }
        //
        // subsets

        // solution 2
        let mut subsets = vec![];
        let mut subset = vec![];

        nums.sort();
        Solution::helper(&nums[..], &mut subset, &mut subsets);
        subsets
    }

    fn helper(nums: &[i32], subset: &mut Vec<i32>, subsets: &mut Vec<Vec<i32>>) {
        subsets.push(subset.clone() as Vec<i32>);

        // all numbers are used
        if nums.is_empty() {
            return;
        }

        // push all combinations which contains nums[i] into subsets,,
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            subset.push(nums[i]);
            Solution::helper(&nums[(i + 1)..], subset, subsets);
            subset.pop();
        }
    }
}

fn main() {
    let input = vec![1, 2, 2];
    let output = vec![
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![2, 2],
        vec![1, 2, 2],
    ];
    assert_eq!(output, Solution::subsets_with_dup(input));
}
