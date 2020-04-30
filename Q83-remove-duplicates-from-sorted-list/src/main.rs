use std::option::Option::Some;

struct Solution {}

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

fn create_linklist(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for num in nums.into_iter().rev() {
        let mut node = ListNode::new(num);

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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut cursor= head.as_mut().unwrap();

        while let Some(node) = cursor.next.as_mut() {
            if cursor.val == node.val {
                cursor.next = node.next.take();
            } else {
                cursor = cursor.next.as_mut().unwrap();
            }
        }

        head
    }
}

fn main() {
    let input = create_linklist(vec![1, 1, 2]);
    let output = create_linklist(vec![1, 2]);
    assert_eq!(output, Solution::delete_duplicates(input));


    let input = create_linklist(vec![1, 1, 2, 3, 3]);
    let output = create_linklist(vec![1, 2, 3]);
    assert_eq!(output, Solution::delete_duplicates(input));


    let input = create_linklist(vec![]);
    let output = create_linklist(vec![]);
    assert_eq!(output, Solution::delete_duplicates(input));
}
