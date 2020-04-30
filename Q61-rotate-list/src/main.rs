use std::option::Option::Some;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution {}


fn create_linklist(v: Vec<i32>) -> Option<Box<ListNode>> {
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


impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = vec![];
        let mut node = head;

        while let Some(_node) = node {
            v.push(_node.val);
            node = _node.next;
        }

        let len = v.len();
        let k = if len > 0 { k as usize % len } else {0};
        let (left, right) = v.split_at_mut(len - k);
        let mut list = right.to_vec();

        list.extend_from_slice(left);
        create_linklist(list)
    }
}


fn linklist_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut cursor = head;
    let mut v = vec![];

    while let Some(node) = cursor {
        v.push(node.val);
        cursor = &node.next;
    }

    v
}


fn main() {
    // Solution::rotate_right(None, 1);

    let input = create_linklist(vec![1, 2, 3, 4, 5]);
    let output = create_linklist(vec![4, 5, 1, 2, 3]);
    assert_eq!(output, Solution::rotate_right(input, 2));


    let input = create_linklist(vec![0, 1, 2]);
    let output = create_linklist(vec![2, 0, 1]);
    assert_eq!(output, Solution::rotate_right(input, 4));


    let input = create_linklist(vec![]);
    let output = create_linklist(vec![]);
    assert_eq!(output, Solution::rotate_right(input, 0));
}
