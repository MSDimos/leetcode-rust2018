use std::rc::Rc;
use std::cell::RefCell;
use leetcode_utils::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::helper(p.as_ref(), q.as_ref())
    }

    fn helper(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if p.xor(q).is_some() {
            false
        } else if p.or(q).is_none() {
            true
        } else {
            let p_ref = p.unwrap().borrow();
            let q_ref = q.unwrap().borrow();

            if p_ref.val != q_ref.val {
                false
            } else {
                Solution::helper(p_ref.left.as_ref(), q_ref.left.as_ref()) &&
                    Solution::helper(p_ref.right.as_ref(), q_ref.right.as_ref())
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
        let input0 = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        let input1 = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(Solution::is_same_tree(input0, input1), true);

        let input0 = TreeNode::create_tree(vec![1, 2], vec![2, 1]);
        let input1 = TreeNode::create_tree(vec![1, 2], vec![1, 2]);
        assert_eq!(Solution::is_same_tree(input0, input1), false);

        let input0 = TreeNode::create_tree(vec![1, 2, 1], vec![2, 1, 1]);
        let input1 = TreeNode::create_tree(vec![1, 1, 2], vec![1, 1, 2]);
        assert_eq!(Solution::is_same_tree(input0, input1), false);

        let input0 = TreeNode::create_tree(vec![1], vec![1]);
        let input1 = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::is_same_tree(input0, input1), true);

        let input0 = TreeNode::create_tree(vec![], vec![]);
        let input1 = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::is_same_tree(input0, input1), false);

        let input0 = TreeNode::create_tree(vec![], vec![]);
        let input1 = TreeNode::create_tree(vec![], vec![]);
        assert_eq!(Solution::is_same_tree(input0, input1), true);
    }
}
