use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;

        for i in v.into_iter().rev() {
            let mut node = ListNode::new(i);

            if head.is_none() {
                head = Some(Box::new(node));
            } else {
                node.next = head;
                head = Some(Box::new(node));
            }
        }

        head
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut cursor = self;
        let mut v = vec![];

        loop {
            v.push(cursor.val);

            if cursor.next.is_some() {
                cursor = cursor.next.as_ref().unwrap();
            } else {
                break;
            }
        }

        v
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn create_tree(pre_order: Vec<i32>, in_order: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        assert_eq!(pre_order.len(), in_order.len(), "pre_order's length must be equal to in_order's");
        TreeNode::create_tree_dfs(&pre_order[..], &in_order[..])
    }

    fn create_tree_dfs(pre_order: &[i32], in_order: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_order.is_empty() || in_order.is_empty() {
            return None;
        }

        let mut node = TreeNode::new(pre_order[0]);

        for i in 0..in_order.len() {
            if in_order[i] == pre_order[0] {
                node.left = TreeNode::create_tree_dfs(&pre_order[1..=i], &in_order[0..i]);
                node.right = TreeNode::create_tree_dfs(&pre_order[(i + 1)..], &in_order[(i + 1)..]);
            }
        }

        Some(Rc::new(RefCell::new(node)))
    }

    pub fn in_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        TreeNode::in_order_dfs(root.as_ref(), &mut result);
        result
    }

    fn in_order_dfs(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node_rc) = node {
            let refer = node_rc.borrow();
            TreeNode::in_order_dfs(refer.left.as_ref(), result);
            result.push(refer.val);
            TreeNode::in_order_dfs(refer.right.as_ref(), result);
        }
    }

    pub fn post_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        TreeNode::post_order_dfs(root.as_ref(), &mut result);
        result
    }

    fn post_order_dfs(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node_rc) = node {
            let refer = node_rc.borrow();
            TreeNode::post_order_dfs(refer.left.as_ref(), result);
            TreeNode::post_order_dfs(refer.right.as_ref(), result);
            result.push(refer.val);
        }
    }

    pub fn pre_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        TreeNode::pre_order_dfs(root.as_ref(), &mut result);
        result
    }

    fn pre_order_dfs(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node_rc) = node {
            let refer = node_rc.borrow();
            result.push(refer.val);
            TreeNode::post_order_dfs(refer.left.as_ref(), result);
            TreeNode::post_order_dfs(refer.right.as_ref(), result);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{ListNode, TreeNode};

    #[test]
    fn linked_list_test() {
        let v = vec![1, 2, 3, 4, 5];
        let l = ListNode::from_vec(v.clone());
        assert_eq!(l.as_ref().unwrap().to_vec(), v);

        let v = vec![5, 4, 3, 2, 1];
        let l = ListNode::from_vec(v.clone());
        assert_eq!(l.as_ref().unwrap().to_vec(), v);
    }

    #[test]
    fn tree_node_test() {
        let tree = TreeNode::create_tree(vec![1, 2, 3], vec![2, 1, 3]);
        assert_eq!(TreeNode::post_order_traversal(tree), vec![2, 3, 1]);
        let tree = TreeNode::create_tree(vec![1, 2, 4, 3, 5], vec![4, 2, 1, 3, 5]);
        assert_eq!(TreeNode::post_order_traversal(tree), vec![4, 2, 5, 3, 1]);
    }
}
