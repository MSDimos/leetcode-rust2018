use leetcode_utils::ListNode;

struct Solution {}

impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut stack = vec![];
        let mut count = 0;
        let mut dummy_node = ListNode::new(0);
        dummy_node.next = head;
        let mut dummy = Some(Box::new(dummy_node));
        let mut cursor = dummy.as_mut().unwrap();

        loop {
            if m - 1 == count {
                for _ in m..=n {
                    let mut nodes = cursor.next.take();

                    if nodes.is_some() {
                        let mut node = nodes.as_mut().unwrap();
                        let mut rest = node.next.take();

                        cursor.next = rest;
                        stack.push(nodes);
                    }
                }

                // pop all nodes from stack and link them all together sequentially
                // why use ptr as raw pointer, because we need ownership of nodes whose type is Option<Box<ListNode>>
                // and we also need `nodes_tail` which is the mutable reference of last node in nodes.
                // It means that we both get a ownership and mutable reference of a same variable, this violate the ownership rules in Rust.
                // So, we need so trick to implement this violation.
                let mut nodes = None;
                let mut nodes_tail = None;
                let mut ptr = 0 as *mut Box<ListNode>;

                while !stack.is_empty() {
                    let node = stack.pop().unwrap();

                    if nodes_tail.is_none() {
                        nodes = node;
                        nodes_tail = nodes.as_mut();
                    } else {
                        let tail = nodes_tail.unwrap();

                        tail.next = node;
                        nodes_tail = tail.next.as_mut();
                    }

                    let tmp = nodes_tail.unwrap();
                    ptr = tmp as *mut Box<ListNode>;
                    nodes_tail = Some(tmp);
                }

                // now, insert nodes after the cursor whose index is m (starts from 1 instead of 0)
                let rest = cursor.next.take();

                cursor.next = nodes;
                // unsafe behavior,
                // convert raw pointer to &mut Box<ListNode> and insert next the rest after nodes_tail
                unsafe {
                    let nodes_tail = &mut *ptr;
                    nodes_tail.next = rest;
                }
                break;
            }

            if cursor.next.is_some() {
                cursor = cursor.next.as_mut().unwrap();
                count += 1;
            } else {
                break;
            }
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use leetcode_utils::ListNode;
    use crate::Solution;

    #[test]
    fn it_works() {
        let input = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let output = ListNode::from_vec(vec![1, 4, 3, 2, 5]);

        assert_eq!(
            Solution::reverse_between(input, 2, 4).unwrap().to_vec(),
            output.unwrap().to_vec()
        );

        let input = ListNode::from_vec(vec![3, 5]);
        let output = ListNode::from_vec(vec![5, 3]);

        assert_eq!(
            Solution::reverse_between(input, 1, 2).unwrap().to_vec(),
            output.unwrap().to_vec()
        );
    }
}
