use core::borrow::BorrowMut;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
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

    pub fn create(list: Vec<i32>) -> Self {
        let mut head = ListNode::new(list[0]);
        let mut node = &mut head;

        for i in list.into_iter().skip(1) {
            node.append(i);
        }

        head
    }

    pub fn print(&self) {
        let mut node = self;

        print!("{}", self.val);
        while let Some(ref b) = node.next {
            print!("->{}", b.val);

            node = b.as_ref();
        }
    }

    pub fn append(&mut self, ele: i32) {
        self.tail().next = Some(Box::new(ListNode::new(ele)));
    }

    pub fn tail(&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            return node.tail();
        }

        self
    }
}



pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        let mut x: i32;
        let mut y: i32;
        let mut current = head.as_mut();

        while p.is_some() || q.is_some() {
            if let Some(v) = p {
                x = v.val;
                p = v.next;
            } else {
                x = 0;
            }
            if let Some(v) = q {
                y = v.val;
                q = v.next;
            } else {
                y = 0;
            }
            let sum = x + y + carry;
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            unsafe {
                current = (*(*(current as *mut ListNode)).next.as_mut().unwrap()).as_mut();
            }
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(1)));
        }

        head.next
    }
}