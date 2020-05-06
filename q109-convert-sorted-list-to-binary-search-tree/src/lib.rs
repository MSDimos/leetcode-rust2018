use leetcode_utils::{ListNode, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            None
        } else {
            let mut len = 0i32;
            let mut node = head.as_ref();

            while node.is_some() {
                len += 1;
                node = node.unwrap().next.as_ref();
            }

            Solution::dichotomy((0, len), &mut head.as_deref())
        }
    }

    fn dichotomy(
        (left, right): (i32, i32),
        stack: &mut Option<&ListNode>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left >= right {
            None
        } else {
            let mid = (left + right) / 2;
            let left = Solution::dichotomy((left, mid), stack);
            let stack_ref = stack.unwrap();
            let mut node = TreeNode::new(stack_ref.val);

            node.left = left;
            *stack = stack_ref.next.as_deref();
            node.right = Solution::dichotomy((mid + 1, right), stack);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::{ListNode, TreeNode};

    #[test]
    fn it_wroks() {
        let answer = TreeNode::create_tree(vec![0, -3, -10, 9, 5], vec![-10, -3, 0, 5, 9]);
        let output = Solution::sorted_list_to_bst(ListNode::from_vec(vec![-10, -3, 0, 5, 9]));

        assert_eq!(output, answer);
    }
}
