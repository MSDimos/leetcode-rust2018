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

    pub fn append(&mut self, ele: i32) {
        let mut node = self as &ListNode;

        while node.next.is_some() {
            if let Some(n) = &node.next {
                node = &**n;
            }
        }

        let mut_node = unsafe {
            let ptr = node as *const ListNode as *mut ListNode;
            &mut *ptr
        };

        mut_node.next = Some(Box::new(ListNode {
            val: ele,
            next: None,
        }));
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut head1 = l1.as_ref();
        let mut head2 = l2.as_ref();
        let mut r = ListNode {
            val: 0,
            next: None,
        };

        while head1.is_some() || head2.is_some() {
            if head1.is_some() && head2.is_none() {
                if let Some(n) = head1 {
                    head1 = n.next.as_ref();
                    r.append(n.val);
                }
            } else if head1.is_none() && head2.is_some() {
                if let Some(n) = head2 {
                    head2 = n.next.as_ref();
                    r.append(n.val);
                }
            } else {
                let n1 = head1.unwrap();
                let n2 = head2.unwrap();

                if n1.val < n2.val {
                    r.append(n1.val);
                    head1 = n1.next.as_ref();
                } else {
                    r.append(n2.val);
                    head2 = n2.next.as_ref();
                }
            }
        }

        r.next
    }
}