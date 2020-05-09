use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Solution::dfs(root.as_ref(), 0, sum)
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, mut sum: i32, target: i32) -> bool {
        if let Some(node_rc) = node {
            let refer = node_rc.borrow();
            sum += refer.val;

            if refer.left.is_none() && refer.right.is_none() {
                sum == target
            } else {
                Solution::dfs(refer.left.as_ref(), sum, target)
                    || Solution::dfs(refer.right.as_ref(), sum, target)
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(
            vec![5, 4, 11, 7, 2, 8, 13, 4, 1],
            vec![7, 11, 2, 4, 5, 13, 8, 4, 1],
        );
        assert_eq!(Solution::has_path_sum(input, 22), true);

        let input = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::has_path_sum(input, 1), true);

        let input = TreeNode::create_tree(vec![1, 2], vec![2, 1]);
        assert_eq!(Solution::has_path_sum(input, 1), false);

        let input = TreeNode::create_tree(vec![], vec![]);
        assert_eq!(Solution::has_path_sum(input, 0), false);
    }
}
