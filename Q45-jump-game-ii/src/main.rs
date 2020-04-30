struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // let mut steps: Vec<i32> = Vec::new();
        //
        // for i in 0..nums.len() {
        //     steps.push(i32::max_value());
        // }
        //
        // steps[0] = 0;
        //
        // for i in 0..nums.len() {
        //     for mut j in 0..i {
        //         j = i - j;
        //         let _i = i as i32;
        //         let _j = j as i32;
        //
        //         // steps[i] = min(steps[j] + 1, steps[i])
        //         if nums[j] + _j >= _i {
        //             steps[i] = steps[i].min(steps[j] + 1);
        //         } else {
        //             continue;
        //         }
        //     }
        // }
        //
        // steps[nums.len() - 1]
        let mut steps = 0;
        let mut end = 0;
        let mut max_pos = 0;

        for i in 0..nums.len() {
            max_pos = max_pos.max(nums[i] + i as i32);

            if i as i32 == end {
                end = max_pos;
                steps += 1;
            }
        }

        steps
    }
}

fn main() {
    assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(5, Solution::jump(vec![1, 1, 1, 1, 1, 1]));
}
