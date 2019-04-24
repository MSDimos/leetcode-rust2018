extern crate Q2_add_two_numbers;

use Q2_add_two_numbers::{ Solution, ListNode };

fn main() {
    let list1 = ListNode::create(vec![2, 4, 3]);
    let list2 = ListNode::create(vec![5, 6, 4]);

    let res = Solution::add_two_numbers(Some(Box::new(list1)), Some(Box::new(list2)));

    res.unwrap_or(Box::new(ListNode::new(0))).print();
}
