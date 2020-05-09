use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::min_height(root.as_deref())
    }

    fn min_height(tree: Option<&RefCell<TreeNode>>) -> i32 {
        if let Some(root) = tree {
            let root = root.borrow();
            let left = root.left.as_deref();
            let right = root.right.as_deref();
            let left_height = Solution::min_height(left);
            let right_height = Solution::min_height(right);

            if left.and(right).is_some() {
                min(left_height, right_height) + 1
            } else {
                // if any child is None, at least one of two heights is zero
                // so we need max height
                max(left_height, right_height) + 1
            }
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
        let input = TreeNode::create_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        assert_eq!(Solution::min_depth(input), 2);

        let input = TreeNode::create_tree(vec![1, 2], vec![1, 2]);
        assert_eq!(Solution::min_depth(input), 2);
    }
}
