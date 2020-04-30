use std::rc::Rc;
use std::cell::RefCell;
use leetcode_utils::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_valid_bst_s1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Hmm.
        let v = TreeNode::in_order_traversal(root);
        let mut sorted = true;

        for i in 1..v.len() {
            if v[i] <= v[i - 1] {
                sorted = false;
                break;
            }
        }

        sorted
    }

    pub fn is_valid_bst_s2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev = i64::min_value();
        Solution::s2_helper(root.as_ref(), &mut prev)
    }

    fn s2_helper(root: Option<&Rc<RefCell<TreeNode>>>, prev: &mut i64) -> bool {
        if let Some(root_rc) = root {
            let borrow = root_rc.borrow();

            if Solution::s2_helper(borrow.left.as_ref(), prev) {
                if *prev < borrow.val as i64 {
                    *prev = borrow.val as i64;

                    return Solution::s2_helper(borrow.right.as_ref(), prev);
                }
            }

            false
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use leetcode_utils::TreeNode;
    use crate::Solution;

    #[test]
    fn solution1_test() {
        let input = TreeNode::create_tree(vec![2, 1, 3], vec![1, 2, 3]);
        assert_eq!(Solution::is_valid_bst_s1(input), true);


        let input = TreeNode::create_tree(vec![5, 1, 4, 3, 6], vec![1, 5, 3, 4, 6]);
        assert_eq!(Solution::is_valid_bst_s1(input), false);

        let input = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::is_valid_bst_s1(input), true);

        let input = TreeNode::create_tree(vec![], vec![]);
        assert_eq!(Solution::is_valid_bst_s1(input), true);

        let input = TreeNode::create_tree(vec![1, 1], vec![1, 1]);
        assert_eq!(Solution::is_valid_bst_s1(input), false);

        let input = TreeNode::create_tree(vec![10, 5, 15, 6, 20], vec![5, 10, 6, 15, 20]);
        assert_eq!(Solution::is_valid_bst_s1(input), false);
    }

    #[test]
    fn solution2_test() {
        let input = TreeNode::create_tree(vec![2, 1, 3], vec![1, 2, 3]);
        assert_eq!(Solution::is_valid_bst_s2(input), true);


        let input = TreeNode::create_tree(vec![5, 1, 4, 3, 6], vec![1, 5, 3, 4, 6]);
        assert_eq!(Solution::is_valid_bst_s2(input), false);

        let input = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::is_valid_bst_s2(input), true);

        let input = TreeNode::create_tree(vec![], vec![]);
        assert_eq!(Solution::is_valid_bst_s2(input), true);

        let input = TreeNode::create_tree(vec![1, 1], vec![1, 1]);
        assert_eq!(Solution::is_valid_bst_s2(input), false);

        let input = TreeNode::create_tree(vec![10, 5, 15, 6, 20], vec![5, 10, 6, 15, 20]);
        assert_eq!(Solution::is_valid_bst_s2(input), false);
    }
}
