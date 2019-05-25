use Q23_merge_k_sorted_lists::{Solution, ListNode};

fn create_list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut list = None;
    let mut cursor = &mut None;

    for (idx, i) in v.into_iter().enumerate() {
        if idx == 0 {
            list = Some(Box::new(ListNode {
                val: i,
                next: None,
            }));
            cursor = &mut list;
        } else {
            if let Some(node) = cursor {
                node.next = Some(Box::new(ListNode {
                    val: i,
                    next: None,
                }));
                cursor = &mut node.next;
            }
        }
    }

    list
}

fn main() {
    let mut v = vec![];

    v.push(create_list(vec![1, 4, 5]));
    v.push(create_list(vec![1, 3, 4]));
    v.push(create_list(vec![2, 6]));

    let r = Solution::merge_k_lists(v);

    println!("{:#?}", r);
}