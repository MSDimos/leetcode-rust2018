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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut node = head.as_ref();
        // I really dislike operating linked list in rust!
        // So I used two vector.
        // But it's easy to operate linked list in C/C++, Java and other script languages
        let mut left = vec![];
        let mut right = vec![];

        while node.is_some() {
            let n = node.unwrap();
            node = n.next.as_ref();

            if n.val < x {
                left.push(n.val);
            } else {
                right.push(n.val);
            }
        }

        left.append(&mut right);
        create_linklist(left)
    }
}

fn main() {
    let input = create_linklist(vec![1, 4, 3, 2, 5, 2]);
    let output = create_linklist(vec![1, 2, 2, 4, 3, 5]);
    assert_eq!(output, Solution::partition(input, 3));


    let input = create_linklist(vec![]);
    let output = create_linklist(vec![]);
    assert_eq!(output, Solution::partition(input, 3));

    let input = create_linklist(vec![]);
    let output = create_linklist(vec![]);
    assert_eq!(output, Solution::partition(input, 3));
}
