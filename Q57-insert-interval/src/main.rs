struct Solution {}

impl Solution {
    fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        Solution::merge(intervals)
    }
}

fn main() {
    assert_eq!(
        vec![vec![1, 5], vec![6, 9]],
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
    )
}
