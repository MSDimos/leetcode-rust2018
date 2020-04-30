struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        // solution 1
        // let mut max_area = 0;
        //
        // for i in 0..heights.len() {
        //     let mut min_height = i32::max_value();
        //
        //     for j in i..heights.len() {
        //         min_height = min_height.min(heights[j]);
        //         max_area = max_area.max(min_height * (j - i + 1) as i32);
        //     }
        // }
        //
        // max_area

        // solution 2
        let mut stack = vec![];
        let mut fixed_heights = vec![0];
        let mut max_area = 0;

        // fixed_heights = [0] + heights + [0]
        // first 0 is index-guard,
        // last 0 is using for triggering calculation if
        // last few elements are monotonically increasing
        fixed_heights.append(&mut heights);
        fixed_heights.push(0);

        for i in 0..fixed_heights.len() {
            while !stack.is_empty() && fixed_heights[stack[stack.len() - 1]] > fixed_heights[i] {
                let last = stack.pop().unwrap();
                let width = (i - stack[stack.len() - 1] - 1) as i32;

                max_area = max_area.max(width * fixed_heights[last]);
            }

            stack.push(i);
        }
        
        max_area
    }
}

fn main() {
    assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3, 4, 1, 2]));
    assert_eq!(1, Solution::largest_rectangle_area(vec![1]));
    assert_eq!(0, Solution::largest_rectangle_area(vec![]));
}
