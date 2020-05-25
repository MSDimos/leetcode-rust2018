use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let stack = vec![root.as_deref()];
        let stack_ptr = {
            &stack as *const Vec<Option<&RefCell<TreeNode>>> as *mut Vec<Option<&RefCell<TreeNode>>>
        };
        let mut cur;
        let mut prev = &None;
        let mut result = vec![];

        while !stack.is_empty() {
            cur = stack.last().unwrap();
            let mut_stack = unsafe { &mut *stack_ptr };
            let cur_refcell = cur.unwrap();
            let ptr = cur_refcell.as_ptr();
            let cur_borrow = cur_refcell.borrow();

            if (cur_borrow.left.is_none() && cur_borrow.right.is_none())
                || (prev.is_some()
                    && (prev == &cur_borrow.left.as_deref()
                        || prev == &cur_borrow.right.as_deref()))
            {
                result.push(cur_borrow.val);
                mut_stack.pop();
                prev = cur;
            } else {
                if cur_borrow.right.is_some() {
                    mut_stack.push(unsafe { &*ptr }.right.as_deref());
                }

                if cur_borrow.left.is_some() {
                    mut_stack.push(unsafe { &*ptr }.left.as_deref());
                }
            }
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
        assert_eq!(Solution::postorder_traversal(input), vec![3, 2, 1]);
    }
}
