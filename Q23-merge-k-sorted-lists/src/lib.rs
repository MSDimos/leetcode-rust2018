// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl Solution {

    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {

        if lists.is_empty() {
            None
        } else if lists.len() == 1 {
            lists.remove(0)
        } else if lists.len() == 2 {
            Solution::merge_two_lists(lists.pop().unwrap_or(None), lists.pop().unwrap_or(None))
        } else {
            Solution::merge_two_lists(Solution::merge_k_lists(lists.split_off(lists.len() / 2)), Solution::merge_k_lists(lists))
        }

    }

    fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut head1 = l1.as_ref();
        let mut head2 = l2.as_ref();
        let mut r = ListNode {
            val: 0,
            next: None,
        };
        let mut r_ptr = &mut r;

        while head1.is_some() || head2.is_some() {
            if head1.is_some() && head2.is_none() {
                if let Some(n) = head1 {
                    head1 = n.next.as_ref();
                    r_ptr.next = Some(Box::new(ListNode {
                        val: n.val,
                        next: None,
                    }));
                    r_ptr = &mut **r_ptr.next.as_mut().unwrap();
                }
            } else if head1.is_none() && head2.is_some() {
                if let Some(n) = head2 {
                    head2 = n.next.as_ref();
                    r_ptr.next = Some(Box::new(ListNode {
                        val: n.val,
                        next: None,
                    }));
                    r_ptr = &mut **r_ptr.next.as_mut().unwrap();
                }
            } else {
                let n1 = head1.unwrap();
                let n2 = head2.unwrap();

                if n1.val < n2.val {
                    r_ptr.next = Some(Box::new(ListNode {
                        val: n1.val,
                        next: None,
                    }));
                    r_ptr = &mut **r_ptr.next.as_mut().unwrap();
                    head1 = n1.next.as_ref();
                } else {
                    r_ptr.next = Some(Box::new(ListNode {
                        val: n2.val,
                        next: None,
                    }));
                    r_ptr = &mut **r_ptr.next.as_mut().unwrap();
                    head2 = n2.next.as_ref();
                }
            }
        }

        r.next
    }
}