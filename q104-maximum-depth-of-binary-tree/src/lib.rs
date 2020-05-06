use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut max_depth = 0;
        let mut sibling_ptrs = vec![root.as_ref().unwrap().as_ptr()];

        loop {
            let mut children_ptrs = vec![];

            for sibling_ptr in sibling_ptrs {
                let sibling = unsafe { &*sibling_ptr };

                if sibling.left.is_some() {
                    children_ptrs.push(sibling.left.as_ref().unwrap().as_ptr());
                }

                if sibling.right.is_some() {
                    children_ptrs.push(sibling.right.as_ref().unwrap().as_ptr());
                }
            }

            max_depth += 1;

            if children_ptrs.is_empty() {
                break;
            } else {
                sibling_ptrs = children_ptrs;
            }
        }

        max_depth
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        assert_eq!(Solution::max_depth(input), 3);

        let input = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(Solution::max_depth(input), 2);
    }
}
