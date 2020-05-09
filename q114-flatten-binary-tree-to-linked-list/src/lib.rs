use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::ptr::null_mut;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (node, _, _) = Solution::dfs(root.take());

        if let Some(node) = node {
            root.replace(node);
        }
    }

    /*
        take ownership of subtree, and flatten it.
        return tree values, they are respectively:
        1. ownership of subtree flattened
        2. raw pointer of head of subtree flattened
        2. raw pointer of tail of subtree flattened
    */
    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> (Option<Rc<RefCell<TreeNode>>>, *mut TreeNode, *mut TreeNode) {
        if let Some(node) = node {
            let node_ptr = node.as_ptr();
            let node_ref = unsafe { &mut *node_ptr };

            if node_ref.left.is_none() && node_ref.right.is_none() {
                (Some(node), node_ptr, node_ptr)
            } else {
                let (left, left_head, left_tail) = Solution::dfs(node_ref.left.take());
                let (right, _, right_tail) = Solution::dfs(node_ref.right.take());

                if left.is_none() {
                    node_ref.right = right;
                    (Some(node), node_ptr, right_tail)
                } else if right.is_none() {
                    node_ref.right = left;
                    (Some(node), node_ptr, left_tail)
                } else {
                    let left_tail_ref = unsafe { &mut *left_tail };

                    left_tail_ref.right = right;
                    node_ref.right = left;
                    (Some(node), left_head, right_tail)
                }
            }
        } else {
            (None, null_mut(), null_mut())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let mut input = TreeNode::create_tree(vec![1, 2, 3, 4, 5, 6], vec![3, 2, 4, 1, 5, 6]);
        let output = TreeNode::create_tree(vec![1, 2, 3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6]);
        Solution::flatten(&mut input);
        assert_eq!(input, output);

        let mut input = TreeNode::create_tree(vec![1], vec![1]);
        let output = TreeNode::create_tree(vec![1], vec![1]);
        Solution::flatten(&mut input);
        assert_eq!(input, output);

        let mut input = TreeNode::create_tree(vec![], vec![]);
        let output = TreeNode::create_tree(vec![], vec![]);
        Solution::flatten(&mut input);
        assert_eq!(input, output);
    }
}
