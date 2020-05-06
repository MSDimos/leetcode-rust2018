use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // optimization: using HashMap to store (value, idx)
        // to decreasing the count of iteration and improve speed
        Solution::dfs(&inorder[..], &postorder[..])
    }

    fn dfs(in_order: &[i32], post_order: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if post_order.is_empty() || in_order.is_empty() {
            return None;
        }

        let mut node = TreeNode::new(post_order[post_order.len() - 1]);

        for i in 0..in_order.len() {
            if in_order[i] == post_order[post_order.len() - 1] {
                node.left = Solution::dfs(&in_order[0..i], &post_order[0..i]);
                node.right =
                    Solution::dfs(&in_order[(i + 1)..], &post_order[i..(post_order.len() - 1)]);
                break;
            }
        }

        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let tree = TreeNode::create_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            tree,
        );
    }
}
