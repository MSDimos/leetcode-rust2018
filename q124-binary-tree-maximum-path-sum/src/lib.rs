use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::min_value();

        Solution::dfs(root, &mut max_sum);
        max_sum
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(root_rc) = root {
            let mut root_mut_borrow = root_rc.borrow_mut();
            let left_sum = max(0, Solution::dfs(root_mut_borrow.left.take(), max_sum));
            let right_sum = max(0, Solution::dfs(root_mut_borrow.right.take(), max_sum));

            *max_sum = max(root_mut_borrow.val + left_sum + right_sum, *max_sum);
            max(left_sum, right_sum) + root_mut_borrow.val
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(Solution::max_path_sum(input), 6);

        let input = TreeNode::create_tree(vec![-10, 9, 20, 15, 7], vec![9, -10, 15, 20, 7]);
        assert_eq!(Solution::max_path_sum(input), 42);

        let input = TreeNode::create_tree(vec![-1, -2, -3], vec![-2, -1, -3]);
        assert_eq!(Solution::max_path_sum(input), -1);
    }
}
