use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
    key: i32,
    val: i32,
}

impl Node {
    fn raw_new(key: i32, val: i32) -> Self {
        Node {
            next: None,
            prev: None,
            key,
            val,
        }
    }

    fn new(key: i32, val: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::raw_new(key, val))))
    }

    fn remove_self(&mut self) -> (i32, i32) {
        let prev_ptr = self.prev.as_ref().unwrap().as_ptr();
        let next_ptr = self.next.as_ref().unwrap().as_ptr();
        let prev = unsafe { &mut *prev_ptr };
        let next = unsafe { &mut *next_ptr };

        prev.next = self.next.take();
        next.prev = self.prev.take();

        (self.key, self.val)
    }
}

struct SizedLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    size: usize,
}

impl SizedLinkedList {
    fn new() -> Self {
        let head = Node::new(0, 0).unwrap();
        let tail = Node::new(0, 0).unwrap();

        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));

        SizedLinkedList {
            head: Some(head),
            tail: Some(tail),
            size: 0,
        }
    }

    #[allow(dead_code)]
    fn pop_head(&mut self) -> Option<(i32, i32)> {
        if self.size > 0 {
            let ptr = self.head.as_deref().unwrap().as_ptr();
            let ptr1 = unsafe { (*ptr).next.as_deref().unwrap().as_ptr() };
            let ptr2 = unsafe { (*ptr1).next.as_deref().unwrap().as_ptr() };

            let node = unsafe { &mut *ptr };
            let node1 = unsafe { &mut *ptr1 };
            let node2 = unsafe { &mut *ptr2 };

            node.next = node1.next.take();
            node2.prev = node1.prev.take();
            self.size -= 1;
            Some((node1.key, node1.val))
        } else {
            None
        }
    }

    fn pop_tail(&mut self) -> Option<(i32, i32)> {
        if self.size > 0 {
            let ptr = self.tail.as_deref().unwrap().as_ptr();
            let ptr1 = unsafe { (*ptr).prev.as_deref().unwrap().as_ptr() };
            let ptr2 = unsafe { (*ptr1).prev.as_deref().unwrap().as_ptr() };

            let node = unsafe { &mut *ptr };
            let node1 = unsafe { &mut *ptr1 };
            let node2 = unsafe { &mut *ptr2 };

            node2.next = node1.next.take();
            node.prev = node1.prev.take();
            self.size -= 1;
            Some((node1.key, node1.val))
        } else {
            None
        }
    }

    fn push_head(&mut self, key: i32, val: i32) -> *mut Node {
        let ptr = self.head.as_deref().unwrap().as_ptr();
        let ptr1 = unsafe { (*ptr).next.as_deref().unwrap().as_ptr() };
        let target = Rc::new(RefCell::new(Node::raw_new(key, val)));
        let target_ptr = target.as_ptr();

        let node = unsafe { &mut *ptr };
        let node2 = unsafe { &mut *ptr1 };
        let node1 = unsafe { &mut *target_ptr };

        node1.next = node.next.take();
        node.next = Some(target);
        node2.prev = Some(Rc::clone(node.next.as_ref().unwrap()));
        node1.prev = Some(Rc::clone(self.head.as_ref().unwrap()));
        self.size += 1;
        target_ptr
    }

    fn remove_node(&mut self, ptr: *mut Node) {
        if self.size > 0 {
            (unsafe { &mut *ptr }).remove_self();
            self.size -= 1;
        }
    }
}

pub struct LRUCache {
    ptr_map: HashMap<i32, *mut Node>,
    inner_linked_list: SizedLinkedList,
    cap: usize,
}

impl LRUCache {
    pub fn new(cap: i32) -> Self {
        LRUCache {
            ptr_map: HashMap::with_capacity(cap as usize),
            inner_linked_list: SizedLinkedList::new(),
            cap: cap as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&ptr) = self.ptr_map.get(&key) {
            let node = unsafe { &*ptr };

            self.inner_linked_list.remove_node(ptr);
            self.ptr_map
                .insert(key, self.inner_linked_list.push_head(key, node.val));
            node.val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&ptr) = self.ptr_map.get(&key) {
            self.inner_linked_list.remove_node(ptr);
            self.ptr_map.remove(&key);
        }

        if self.ptr_map.len() >= self.cap {
            if let Some((key, _)) = self.inner_linked_list.pop_tail() {
                self.ptr_map.remove(&key);
            }
        }

        let ptr = self.inner_linked_list.push_head(key, value);
        self.ptr_map.insert(key, ptr);
    }
}

impl fmt::Display for SizedLinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cusor = self.head.as_deref();
        let mut result = vec![];

        while cusor.is_some() {
            let node = unsafe { &*cusor.unwrap().as_ptr() };

            result.push((node.key, node.val));
            cusor = node.next.as_deref();
        }

        writeln!(f, "{:?}", result)
    }
}

#[cfg(test)]
mod tests {
    use crate::LRUCache;

    #[test]
    fn it_works() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);

        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(2), -1);
        cache.put(2, 6);
        assert_eq!(cache.get(1), -1);
        cache.put(1, 5);
        cache.put(1, 2);
        assert_eq!(cache.get(1), 2);
        assert_eq!(cache.get(2), 6);
    }
}
