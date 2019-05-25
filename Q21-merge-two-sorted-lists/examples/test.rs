use Q21_merge_two_sorted_lists::{ Solution, ListNode };

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
    let mut list = create_list(vec![]);
    let mut list2 = create_list(vec![1, 2, 2]);

    let r = Solution::merge_two_lists(list, list2);
    println!("{:#?}", r);
}