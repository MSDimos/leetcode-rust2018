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
        if head.is_none() { return head; }

        let mut duplicated_value = head.as_ref().unwrap().val - 1;
        let mut fake = Some(Box::new(ListNode { val: duplicated_value, next: head, }));
        // let mut node = &mut head;
        let mut cursor = fake.as_mut().unwrap();


        while let Some(node) = cursor.next.as_mut() {
            if node.val == duplicated_value {
                cursor.next = node.next.take();
            } else {
                if node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                    duplicated_value = node.val;
                } else {
                    cursor = cursor.next.as_mut().unwrap();
                }
            }
        }

        fake.unwrap().next
    }
}

// method 2

// impl Solution {
//     pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         if head.is_none() { return head; }
//
//         let mut duplicated_value = head.as_ref().unwrap().val - 1;
//         let mut node = &mut head;
//
//         loop {
//             // acceptable
//             match node {
//                 Some(n) if n.val == duplicated_value => *node = n.next.take(),
//                 Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => duplicated_value = n.val,
//                 Some(n) => node = &mut n.next,
//                 _ => return head,
//             }
//         }
//
//         head
//     }
// }


fn main() {
    let input = create_linklist(vec![1, 2, 3, 3, 4, 4, 5]);
    let output = create_linklist(vec![1, 2, 5]);
    assert_eq!(output, Solution::delete_duplicates(input));


    let input = create_linklist(vec![1, 1, 1, 2, 3]);
    let output = create_linklist(vec![2, 3]);
    assert_eq!(output, Solution::delete_duplicates(input));

    let input = create_linklist(vec![1, 1, 1, 1]);
    let output = create_linklist(vec![]);
    assert_eq!(output, Solution::delete_duplicates(input));

    let input = create_linklist(vec![0, 1, 1, 1]);
    let output = create_linklist(vec![0]);
    assert_eq!(output, Solution::delete_duplicates(input));
}
