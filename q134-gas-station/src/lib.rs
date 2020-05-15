pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut rest = 0;
        let mut rest_partial = 0;
        let mut start_idx = 0;

        for i in 0..gas.len() {
            rest += gas[i] - cost[i];
            rest_partial += gas[i] - cost[i];

            // if rest_partial < 0, it means cost[i] > gas[i] - prev_rest_partial
            // it means if run starts from prev_start_idx, it will never arrivals station-i
            // so we need to try to start from station-(i + 1)
            if rest_partial < 0 {
                rest_partial = 0;
                start_idx = i + 1;
            }
        }

        if rest >= 0 {
            start_idx as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);

        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 0);

        let gas = vec![0];
        let cost = vec![0];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 0);

        let gas = vec![1];
        let cost = vec![0];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 0);

        let gas = vec![1, 2];
        let cost = vec![2, 1];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 1);
    }
}
