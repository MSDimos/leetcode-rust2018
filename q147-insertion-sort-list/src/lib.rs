use leetcode_utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cursor = head;
        let mut guard = ListNode::new(0);

        while let Some(mut target) = cursor {
            cursor = target.next.take();
            let mut current = &mut guard;

            while current.next.is_some() && current.next.as_ref().unwrap().val < target.val {
                current = current.next.as_mut().unwrap();
            }

            target.next = current.next.take();
            current.next = Some(target);
        }

        guard.next
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
            Solution::insertion_sort_list(input),
            ListNode::from_vec(vec![1, 2, 3, 4])
        );
    }
}
