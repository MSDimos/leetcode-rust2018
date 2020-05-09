use leetcode_utils::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        let mut path = vec![];

        Solution::dfs(root.as_ref(), (0, sum), (&mut path, &mut paths));
        paths
    }

    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        (mut sum, target): (i32, i32),
        (path, paths): (&mut Vec<i32>, &mut Vec<Vec<i32>>),
    ) {
        if let Some(node_rc) = node {
            let refer = node_rc.borrow();

            path.push(refer.val);
            sum += refer.val;

            if refer.left.is_none() && refer.right.is_none() {
                if sum == target {
                    paths.push(path.clone() as Vec<i32>);
                }
            } else {
                Solution::dfs(refer.left.as_ref(), (sum, target), (path, paths));
                Solution::dfs(refer.right.as_ref(), (sum, target), (path, paths));
            }

            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;

    #[test]
    fn it_works() {
        let input = TreeNode::create_tree(
            vec![5, 4, 11, 7, 2, 8, 13, 4, 5, 1],
            vec![7, 11, 2, 4, 5, 13, 8, 5, 4, 1],
        );
        assert_eq!(
            Solution::path_sum(input, 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5],]
        );

        let input = TreeNode::create_tree(vec![1], vec![1]);
        assert_eq!(Solution::path_sum(input, 1), vec![vec![1]]);

        let input = TreeNode::create_tree(vec![1, 2], vec![2, 1]);
        assert_eq!(Solution::path_sum(input, 1), vec![] as Vec<Vec<i32>>);

        let input = TreeNode::create_tree(vec![], vec![]);
        assert_eq!(Solution::path_sum(input, 0), vec![] as Vec<Vec<i32>>);
    }
}
