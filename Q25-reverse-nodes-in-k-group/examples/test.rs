use Q25_reverse_nodes_in_k_group::{ Solution, ListNode };

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
    let list = create_list(vec![1, 2]);
    let r = Solution::reverse_k_group(list, 0);

    println!("{:#?}", r);
}