use std::rc::Rc;
use std::cell::RefCell;
use leetcode_utils::TreeNode;

struct Solution {}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            vec![]
        } else {
            Solution::dfs(1, n as usize)
        }
    }

    fn dfs(start: usize, end: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut trees = vec![];

        if start > end {
            trees.push(None);
            return trees;
        }

        for i in start..=end {
            let left_trees = Solution::dfs(start, i - 1);
            let right_trees = Solution::dfs(i + 1, end);

            for left_tree in left_trees.iter() {
                for right_tree in right_trees.iter() {
                    let node = Some(Rc::new(RefCell::new(TreeNode {
                        val: i as i32,
                        left: left_tree.clone(),
                        right: right_tree.clone(),
                    })));
                    trees.push(node);
                }
            }
        }

        trees
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::rc::Rc;
    use std::cell::RefCell;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(Solution::generate_trees(0), vec![]);
        assert_eq!(Solution::generate_trees(1), vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))]);
        assert_eq!(
            Solution::generate_trees(2),
            vec![
                TreeNode::create_tree(vec![1, 2], vec![1, 2]),
                TreeNode::create_tree(vec![2, 1], vec![1, 2]),
            ]
        );
    }
}
