use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

enum Direction {
    Left2Right = 0,
    Right2Left = 1,
}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut level_traversal = vec![];
        let mut sibling_ptrs = vec![root.as_ref().unwrap().as_ptr()];
        let mut direction = Direction::Left2Right;

        loop {
            let mut children_ptrs = vec![];
            let mut values = vec![];

            for sibling_ptr in sibling_ptrs {
                let sibling = unsafe { &*sibling_ptr };

                if sibling.left.is_some() {
                    children_ptrs.push(sibling.left.as_ref().unwrap().as_ptr());
                }

                if sibling.right.is_some() {
                    children_ptrs.push(sibling.right.as_ref().unwrap().as_ptr());
                }

                values.push(sibling.val);
            }

            match direction {
                Direction::Left2Right => {
                    direction = Direction::Right2Left;
                    level_traversal.push(values);
                }
                Direction::Right2Left => {
                    direction = Direction::Left2Right;
                    values.reverse();
                    level_traversal.push(values);
                }
            }

            if children_ptrs.is_empty() {
                break;
            } else {
                sibling_ptrs = children_ptrs;
            }
        }

        level_traversal
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        assert_eq!(
            Solution::zigzag_level_order(input),
            vec![vec![3], vec![20, 9], vec![15, 7]],
        );

        let input = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(
            Solution::zigzag_level_order(input),
            vec![vec![1], vec![3, 2]],
        );
    }
}
