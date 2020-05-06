use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::create_tree(preorder, inorder)
    }
}

#[cfg(test)]
mod tests {
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let tree = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(TreeNode::post_order_traversal(tree), vec![2, 3, 1]);
        let tree = TreeNode::create_tree(vec![1, 2, 4, 3, 5], vec![4, 2, 1, 3, 5]);
        assert_eq!(TreeNode::post_order_traversal(tree), vec![4, 2, 5, 3, 1]);
    }
}
