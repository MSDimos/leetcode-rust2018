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

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_some() {
            let mut head = *head.unwrap();

            if head.next.is_some() {
                let mut tail = *head.next.unwrap();

                head.next = Solution::swap_pairs(tail.next);
                tail.next = Some(Box::new(head));

                Some(Box::new(tail))
            } else {
                Some(Box::new(head))
            }
        } else {
            None
        }
    }
}