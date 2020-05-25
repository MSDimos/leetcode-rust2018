use leetcode_utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (lhs, rhs) = Self::split(head);
        match (lhs, rhs) {
            (None, None) => None,
            (lhs, None) => lhs,
            (None, rhs) => rhs,
            (lhs, rhs) => Self::merge(Self::sort_list(lhs), Self::sort_list(rhs)),
        }
    }

    // split list into two list at center position
    fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut lhs = None;
        let mut rhs = None;
        let mut coin = true;

        while let Some(mut head_node) = head {
            let rest = head_node.next.take();

            if coin {
                head_node.next = lhs.take();
                lhs = Some(head_node);
            } else {
                head_node.next = rhs.take();
                rhs = Some(head_node);
            }

            head = rest;
            coin = !coin;
        }

        (lhs, rhs)
    }

    // merge two list in order of increasing
    fn merge(lhs: Option<Box<ListNode>>, rhs: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (lhs, rhs) {
            (None, None) => None,
            (node, None) | (None, node) => node,
            (Some(h), Some(t)) => {
                let (mut small, large) = if h.val < t.val { (h, t) } else { (t, h) };
                let successor = small.next.take();
                small.next = Self::merge(successor, Some(large));
                Some(small)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::ListNode;

    #[test]
    fn it_works() {
        let input = ListNode::from_vec(vec![4, 2, 1, 3]);
        assert_eq!(
            Solution::sort_list(input),
            ListNode::from_vec(vec![1, 2, 3, 4])
        );
    }
}
