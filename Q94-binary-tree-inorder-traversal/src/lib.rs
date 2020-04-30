use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use leetcode_utils::TreeNode;
use std::borrow::Borrow;

struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // recursive solution
        // let mut result = vec![];
        //
        // Solution::dfs(root.as_ref(), &mut result);
        // result

        // iterative solution
        let mut result = vec![];
        let mut stack = vec![];
        let mut cursor = root.as_ref();

        while cursor.is_some() || !stack.is_empty() {
            while let Some(cursor_rc) = cursor {
                let ptr = cursor_rc.as_ptr();
                let refer = unsafe { &*ptr };
                stack.push(ptr);
                cursor = refer.left.as_ref();
            }

            let ptr = stack.pop().unwrap();
            result.push(unsafe { (&*ptr).val });
            cursor = unsafe { (&*ptr).right.as_ref() };
        }

        result
    }

    // fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    //     if let Some(node_rc) = node {
    //         let refer = node_rc.borrow();
    //         Solution::dfs(refer.left.as_ref(), result);
    //         result.push(refer.val);
    //         Solution::dfs(refer.right.as_ref(), result);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        let tree = TreeNode::create_tree(vec![1, 2, 3], vec![1, 3, 2]);
        assert_eq!(Solution::inorder_traversal(tree), vec![1, 3, 2]);
    }
}
