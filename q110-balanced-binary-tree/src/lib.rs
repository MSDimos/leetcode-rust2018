use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let tree = root.as_deref();

        Solution::is_balance_or_height(tree).is_some()
    }

    fn is_balance_or_height(tree: Option<&RefCell<TreeNode>>) -> Option<i32> {
        if let Some(root) = tree {
            let root = root.borrow();
            let left_height = Solution::is_balance_or_height(root.left.as_deref());
            let right_height = Solution::is_balance_or_height(root.right.as_deref());

            if left_height.and(right_height).is_none() {
                None
            } else {
                let lh = left_height.unwrap();
                let rh = right_height.unwrap();

                if (lh - rh).abs() > 1 {
                    None
                } else {
                    Some(lh.max(rh) + 1)
                }
            }
        } else {
            Some(0)
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
        assert_eq!(Solution::is_balanced(input), true);

        let input = TreeNode::create_tree(vec![1, 2, 3, 4, 4, 3, 2], vec![4, 3, 4, 2, 3, 1, 2]);
        assert_eq!(Solution::is_balanced(input), false);
    }
}
