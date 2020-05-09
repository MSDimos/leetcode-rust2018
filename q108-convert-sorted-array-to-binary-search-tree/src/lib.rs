use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::dichotomy(&nums[..])
    }

    fn dichotomy(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let mid_idx = nums.len() / 2;
            let mid_num = nums[mid_idx];
            let mut node = TreeNode::new(mid_num);

            node.left = Solution::dichotomy(&nums[..mid_idx]);
            node.right = Solution::dichotomy(&nums[(mid_idx + 1)..]);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(vec![0, -3, -10, 9, 5], vec![-10, -3, 0, 5, 9]);
        let output = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);

        assert_eq!(output, input);
    }
}
