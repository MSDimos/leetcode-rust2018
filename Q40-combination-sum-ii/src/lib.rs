//pub struct Solution;
//
//static mut PREV_LAST: i32 = 0;
//static mut PREV_LEN: usize = 0;
//
//impl Solution {
//    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//        let mut result = vec![];
//
//        // candidates.sort();
//        Solution::back(&candidates, 0, 0, target, &mut vec![], &mut result);
//        result
//    }
//
//    // prev_last and prev_len is using for avoiding duplicated result
//    pub fn back(candidates: &Vec<i32>, idx: usize, mut sum: i32, target: i32, stack: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
//        for i in idx..candidates.len() {
//            let num = candidates[i];
//            let s = sum + num;
//
//            if s > target {
//                // break;
//            } else {
//                stack.push(num.clone());
//                if s == target {
//                    unsafe {
//                        if PREV_LAST != num && PREV_LEN != stack.len() {
//                            result.push(stack.clone());
//                            PREV_LAST = num;
//                            PREV_LEN = stack.len();
//                        }
//                    }
//                } else if s < target {
//                    Solution::back(candidates, i + 1, s, target, stack, result);
//                }
//                stack.pop();
//            }
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use crate::Solution;
//    #[test]
//    fn it_works() {
//        let r = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
//        assert_eq!(r, vec![
//            vec![1, 7],
//            vec![1, 2, 5],
//            vec![2, 6],
//            vec![1, 1, 6],
//        ]);
//
//        let r = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
//        assert_eq!(r, vec![
//            vec![1, 2, 2],
//            vec![5],
//        ]);
//    }
//}

pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];

        candidates.sort();
        Solution::backtrack(&candidates, 0, target, &mut vec![], &mut result);
        result
    }

    fn backtrack(candidates: &Vec<i32>, idx: usize, target: i32, stack: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        let mut prev = -1;

        if target == 0 {
            result.push(stack.clone());
            return;
        }

        for i in idx..candidates.len() {
            if candidates[i] <= target && candidates[i] != prev {
                stack.push(candidates[i]);
                Solution::backtrack(candidates, i + 1, target - candidates[i], stack, result);
                prev = stack.pop().unwrap_or(-1);
            } else if candidates[i] > target {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        let r = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        assert_eq!(r, vec![
            vec![1, 7],
            vec![1, 2, 5],
            vec![2, 6],
            vec![1, 1, 6],
        ]);

        let r = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
        assert_eq!(r, vec![
            vec![1, 2, 2],
            vec![5],
        ]);
    }
}