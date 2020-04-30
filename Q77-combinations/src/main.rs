struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if n == 0 || k == 0 {
            return vec![];
        }
        // method 1
        // let mut result = vec![];
        // let mut combination = vec![1; k as usize];
        // let mut idx = (0, 0);
        //
        // loop {
        //     if combination[idx] > n {
        //         if idx == 0 {
        //             break;
        //         }
        //
        //         idx -= 1;
        //         combination[idx] += 1;
        //     } else if idx == k as usize - 1 {
        //         result.push(combination.clone());
        //         combination[idx] += 1;
        //     } else {
        //         idx += 1;
        //         combination[idx] = combination[idx - 1] + 1;
        //     }
        // }

        // method 2
        // let mut result = vec![];
        // let mut nums = vec![];
        //
        // for i in 1..=k {
        //     nums.push(i);
        // }
        //
        // nums.push(n + 1);
        //
        // let mut j = 0;
        //
        // while j < k as usize {
        //     result.push((&nums[..k as usize]).clone().to_vec());
        //     j = 0;
        //
        //     while j < k as usize && nums[j + 1] == nums[j] + 1 {
        //         nums[j] = j as i32 + 1;
        //         j += 1;
        //     }
        //
        //     nums[j] += 1;
        // }
        //
        // result


        // method 3
        let mut result = vec![];
        let mut combination = vec![];

        Solution::helper(1, (n as usize, k as usize), &mut combination, &mut result);
        result
    }

    fn helper(start: usize, (n, k): (usize, usize), combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if combination.len() == k {
            result.push(combination.clone() as Vec<i32>);
        } else {
            if n >= start && n - start + 1 >= k - combination.len() {
                for i in start..=n {
                    combination.push(i as i32);
                    Solution::helper(i + 1, (n, k), combination, result);
                    combination.pop();
                }
            }
        }
    }
}

fn main() {
    // method 1
    // let output = vec![
    //     vec![1, 2],
    //     vec![1, 3],
    //     vec![1, 4],
    //     vec![2, 3],
    //     vec![2, 4],
    //     vec![3, 4],
    // ];
    // assert_eq!(output, Solution::combine(4, 2));

    // method 2
    // let output = vec![
    //     vec![1, 2],
    //     vec![1, 3],
    //     vec![2, 3],
    //     vec![1, 4],
    //     vec![2, 4],
    //     vec![3, 4],
    // ];
    // assert_eq!(output, Solution::combine(4, 2));

    // method 3
    let output = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ];
    assert_eq!(output, Solution::combine(4, 2));
}
