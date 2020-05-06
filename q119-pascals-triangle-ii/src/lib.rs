pub struct Solution {}

impl Solution {
    pub fn get_row(num_rows: i32) -> Vec<i32> {
        let mut prev = vec![];

        for i in 1..=(num_rows + 1) as usize {
            let mut row = vec![1; i];

            if i >= 3 {
                for j in 1..(i - 1) {
                    row[j] = prev[j - 1] + prev[j];
                }
            }

            prev = row;
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
