pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = vec![];

        for i in 1..=num_rows as usize {
            let mut row = vec![1; i];

            if i >= 3 {
                let prev = output.last().unwrap();

                for j in 1..(i - 1) {
                    row[j] = prev[j - 1] + prev[j];
                }
            }

            output.push(row);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
