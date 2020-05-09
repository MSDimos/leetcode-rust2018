use std::rc::Rc;
use std::cell::RefCell;
use leetcode_utils::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root_rc) = root {
            let left = root_rc.borrow_mut().left.take();
            let right = root_rc.borrow_mut().right.take();

            Solution::dfs(left.as_ref(), right.as_ref())
        } else {
            true
        }
    }

    fn dfs(left: Option<&Rc<RefCell<TreeNode>>>, right: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if left.or(right).is_none() {
            true
        } else if left.xor(right).is_some() {
            false
        } else {
            let left = left.unwrap().borrow();
            let right = right.unwrap().borrow();

            if left.val == right.val {
                Solution::dfs(left.left.as_ref(), right.right.as_ref()) &&
                    Solution::dfs(left.right.as_ref(), right.left.as_ref())
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use leetcode_utils::TreeNode;
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(vec![1, 2, 3, 4, 2, 4, 3], vec![3, 2, 4, 1, 4, 2, 3]);
        assert_eq!(Solution::is_symmetric(input), true);

        let input = TreeNode::create_tree(vec![1, 2, 3, 2, 3], vec![2, 3, 1, 2, 3]);
        assert_eq!(Solution::is_symmetric(input), false);

        let input = TreeNode::create_tree(vec![], vec![]);
        assert_eq!(Solution::is_symmetric(input), true);

        let input = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::is_symmetric(input), true);

        let input = TreeNode::create_tree(vec![1, 2], vec![2, 1]);
        assert_eq!(Solution::is_symmetric(input), false);

        let input = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(Solution::is_symmetric(input), false);
    }
}
