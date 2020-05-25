use leetcode_utils::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if let Some(head) = head.as_deref_mut() {
            let v = head.to_vec();
            let mut result = vec![];
            let mut left = 0;
            let mut right = v.len() - 1;

            while left < right {
                result.push(v[left]);
                result.push(v[right]);
                left += 1;
                right -= 1;
            }

            if left == right {
                result.push(v[left]);
            }

            head.val = result.remove(0);
            let new_head2 = ListNode::from_vec(result);
            head.next.replace(new_head2.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use leetcode_utils::ListNode;

    #[test]
    fn it_works() {
        let mut input = ListNode::from_vec(vec![1, 2, 3, 4]);
        let output = ListNode::from_vec(vec![1, 4, 2, 3]);

        Solution::reorder_list(&mut input);
        assert_eq!(input, output);

        let mut input = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let output = ListNode::from_vec(vec![1, 5, 2, 4, 3]);

        Solution::reorder_list(&mut input);
        assert_eq!(input, output);
    }
}
