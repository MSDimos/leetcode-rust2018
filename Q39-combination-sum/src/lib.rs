pub struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];

        candidates.sort();
        Solution::back(&candidates, 0, target, &mut vec![], &mut result);
        result
    }

    pub fn back(
        candidates: &Vec<i32>,
        mut sum: i32,
        target: i32,
        stack: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        for num in candidates {
            let s = sum + num.clone();

            if s > target {
                break;
            } else {
                if Some(num) >= stack.last() {
                    stack.push(num.clone());
                    if s == target {
                        result.push(stack.clone());
                    } else if s < target {
                        Solution::back(candidates, s, target, stack, result);
                    }
                    stack.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        let r = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        assert_eq!(r, vec![vec![2, 2, 3], vec![7]]);
    }
}
