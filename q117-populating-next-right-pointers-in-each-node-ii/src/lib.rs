#![allow(dead_code)]

use std::ptr::null_mut;

struct Node {
    val: i32,
    left: *mut Node,
    right: *mut Node,
    next: *mut Node,
}

pub struct Solution {}
struct SolutionInner {
    prev: *mut Node,
    left_most: *mut Node,
}

impl SolutionInner {
    unsafe fn new() -> Self {
        SolutionInner {
            prev: null_mut(),
            left_most: null_mut(),
        }
    }

    unsafe fn process_child(&mut self, node: *mut Node) {
        if node.is_null() {
            // If it's null, it means this child node is the first node
            // we have encountered on the next level, so, we
            // set the leftmost pointer
            if self.prev.is_null() {
                self.left_most = node;
            } else {
                // If the "prev" pointer is alread set i.e. if we
                // already found atleast one node on the next level,
                // setup its next pointer
                (*self.prev).next = node;
            }

            self.prev = node;
        }
    }

    unsafe fn connect(&mut self, node: *mut Node) {
        if node.is_null() {
            return;
        }

        self.left_most = node;
        let mut curr;

        while !self.left_most.is_null() {
            self.prev = null_mut();
            curr = self.left_most;
            self.left_most = null_mut();

            while !curr.is_null() {
                self.process_child((*curr).left);
                self.process_child((*curr).right);
                curr = (*curr).next;
            }
        }
    }
}

impl Solution {
    unsafe fn connect(node: *mut Node) {
        let mut solution = SolutionInner::new();
        solution.connect(node);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // same as q116, need to test
        // I doesn't test it
    }
}
