use Q19_remove_nth_node_from_end_of_list::{ ListNode, Solution };

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

struct Test {
    a: String,
}

fn main() {
    let head = create_list(vec![1, 2, 3, 4, 5]);
    let list = Solution::remove_nth_from_end(head, 3);

    println!("{:#?}", list);
}