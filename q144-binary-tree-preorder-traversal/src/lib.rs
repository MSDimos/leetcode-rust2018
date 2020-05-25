use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::option::Option::Some;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![root.as_deref()];

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let noed_ref = unsafe { &*node.as_ptr() };

                result.push(noed_ref.val);

                if noed_ref.right.is_some() {
                    stack.push(noed_ref.right.as_deref());
                }

                if noed_ref.left.is_some() {
                    stack.push(noed_ref.left.as_deref());
                }
            }
        }

        result
    }

    pub fn preorder_traversal_s2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        if root.is_none() {
            return result;
        }

        let mut stack = vec![root.as_deref()];
        let mut cursor = root.as_deref();

        while !stack.is_empty() {
            while let Some(cursor_rc) = cursor {
                let cursor_ref = unsafe { &*cursor_rc.as_ptr() };

                result.push(cursor_ref.val);
                stack.push(Some(cursor_rc));
                cursor = cursor_ref.left.as_deref();
            }

            cursor = stack.pop().unwrap();
            cursor = unsafe { &*cursor.unwrap().as_ptr() }.right.as_deref();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(vec![1, 2, 3], vec![1, 3, 2]);
        assert_eq!(Solution::preorder_traversal(input), vec![1, 2, 3]);
    }

    #[test]
    fn it_works_s2() {
        let input = TreeNode::create_tree(vec![1, 2, 3], vec![1, 3, 2]);
        assert_eq!(Solution::preorder_traversal_s2(input), vec![1, 2, 3]);
    }
}
