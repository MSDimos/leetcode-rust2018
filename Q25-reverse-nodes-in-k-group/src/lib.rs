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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k != 0 {
            Solution::reverse_k_group_with_ref(&mut head, k);
        }

        head
    }

    pub fn reverse_k_group_with_ref(head: &mut Option<Box<ListNode>>, k: i32) {
        let mut stack = vec![];
        let mut i = 0;
        let ptr = head as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
        let mut cur = unsafe {
            &mut *ptr
        };

        while cur.is_some() && i < k {
            i += 1;
            if let Some(node) = cur {
                stack.push(node.val);
                cur = &mut node.next;
            }
        }

        let mut cursor = unsafe {
            &mut *ptr
        };
        let mut j = 0;

        if i == k {
            while j < k {
                j += 1;
                if let Some(node) = cursor {
                    node.val = stack.pop().unwrap();
                    cursor = &mut node.next;
                }
            }
        }

        if cur.is_some() {
            Solution::reverse_k_group_with_ref(cur, k);
        }
    }
}