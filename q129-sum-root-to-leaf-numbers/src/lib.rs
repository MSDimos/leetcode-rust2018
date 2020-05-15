use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            let mut sum = 0;
            queue.push_back((vec![root.as_ptr()], root.borrow().val));

            while !queue.is_empty() {
                let len = queue.len();

                for _ in 0..len {
                    let (mut node_ptrs, num) = queue.pop_front().unwrap();
                    let node_ptr = node_ptrs.last().unwrap();
                    let node = unsafe { &**node_ptr };

                    if node.left.is_none() && node.right.is_none() {
                        sum += num;
                    }

                    if node.left.is_some() {
                        let left = node.left.as_ref().unwrap();

                        node_ptrs.push(left.as_ptr());
                        queue.push_back((node_ptrs.clone(), num * 10 + left.borrow().val));
                        node_ptrs.pop();
                    }

                    if node.right.is_some() {
                        let right = node.right.as_ref().unwrap();

                        node_ptrs.push(right.as_ptr());
                        queue.push_back((node_ptrs.clone(), num * 10 + right.borrow().val));
                        node_ptrs.pop();
                    }
                }
            }

            sum
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
        let input = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(Solution::sum_numbers(input), 25);

        let input = TreeNode::create_tree(vec![4, 9, 5, 1, 0], vec![5, 9, 1, 4, 0]);
        assert_eq!(Solution::sum_numbers(input), 1026);

        let input = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::sum_numbers(input), 1);

        let input = TreeNode::create_tree(vec![], vec![]);
        assert_eq!(Solution::sum_numbers(input), 0);
    }
}
