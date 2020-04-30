use std::rc::Rc;
use std::cell::RefCell;
use leetcode_utils::TreeNode;

struct Solution {}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut two_element_ptrs = [0 as *mut TreeNode, 0 as *mut TreeNode];
        let mut prev_ptr = 0 as *mut TreeNode;

        Solution::in_order(root.as_mut(), &mut two_element_ptrs, &mut prev_ptr);

        unsafe {
            let tmp = (&*two_element_ptrs[0]).val;
            (&mut *two_element_ptrs[0]).val = (&*two_element_ptrs[1]).val;
            (&mut *two_element_ptrs[1]).val = tmp;
        }
    }

    fn in_order(root: Option<&mut Rc<RefCell<TreeNode>>>,two_ptrs: &mut [*mut TreeNode; 2], prev_ptr: &mut *mut TreeNode) {
        if let Some(root_rc) = root {
            let root_ptr = root_rc.as_ptr();
            let mut root_bm = root_rc.borrow_mut();

            Solution::in_order(root_bm.left.as_mut(), two_ptrs, prev_ptr);

            // find two target nodes that don't meet the monotonically increasing rule
            let prev_node = unsafe { &**prev_ptr };
            if *prev_ptr as usize != 0 && prev_node.val > root_bm.val {
                if two_ptrs[0] as usize == 0 {
                    two_ptrs[0] = *prev_ptr;
                }

                two_ptrs[1] = root_ptr;
            }

            *prev_ptr = root_ptr;
            Solution::in_order(root_bm.right.as_mut(), two_ptrs, prev_ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use leetcode_utils::TreeNode;
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut input = TreeNode::create_tree(vec![1, 3, 2], vec![3, 2, 1]);
        let output = TreeNode::create_tree(vec![3, 1, 2], vec![1, 2, 3]);
        Solution::recover_tree(&mut input);
        assert_eq!(input, output);

        let mut input = TreeNode::create_tree(vec![3, 1, 4, 2], vec![1, 3, 2, 4]);
        let output = TreeNode::create_tree(vec![2, 1, 4, 3], vec![1, 2, 3, 4]);
        Solution::recover_tree(&mut input);
        assert_eq!(input, output);

        let mut input = TreeNode::create_tree(vec![1, 2], vec![2, 1]);
        let output = TreeNode::create_tree(vec![2, 1], vec![1, 2]);
        Solution::recover_tree(&mut input);
        assert_eq!(input, output);
    }
}
