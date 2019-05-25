// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Don't ask me why don't use RefCell,
        // RefCell<Option<Box<ListNode>>> is so fucking ugly
        let raw_head: *mut _ = &mut head;
        let mut first = unsafe { &mut *raw_head };
        let mut second = unsafe { &mut *raw_head };
        let mut i = 0;

        while i < n {
            if let Some(node) = second {
                i += 1;
                second = &mut node.next;
            } else {
                break;
            }
        }

        while second.is_some() {
            if let Some(node) = first {
                first = &mut node.next;
            }

            if let Some(node) = second {
                second = &mut node.next;
            }
        }

        let raw = first as *mut Option<Box<ListNode>>;

        // using mem::replace() to remove the n.th element
        // why use this? Because of ownership system of Rust is complex.
        // If you want to remove element, it means that you will need the ownership of it
        // but it's conflict with this question: only once iterate
        if let Some(node) = unsafe { &mut *raw } {
            std::mem::replace(unsafe { &mut *raw }, node.next.take());
        }

        head
    }
}