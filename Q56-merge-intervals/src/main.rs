
struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        for interval in intervals {
            if result.is_empty() {
                result.push(interval);
            } else {
                let idx = result.len() - 1;
                if interval[0] > result[idx][1] {
                    result.push(interval);
                } else {
                    result[idx][1] = result[idx][1].max(interval[1]);
                }
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        vec![vec![1,6],vec![8,10], vec![15,18]],
        Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]])
    );
    assert_eq!(
        vec![vec![1, 5]],
        Solution::merge(vec![vec![1, 4], vec![4, 5]])
    )
}
