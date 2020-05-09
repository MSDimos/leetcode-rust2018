#![allow(dead_code)]

struct Node {
    val: i32,
    left: *mut Node,
    right: *mut Node,
    next: *mut Node,
}

pub struct Solution {}

impl Solution {
    unsafe fn connect(node: *mut Node) {
        if !node.is_null() {
            (*(*node).left).next = (*node).right;

            if !(*node).next.is_null() {
                (*(*node).right).next = (*(*node).next).left;
            }

            Solution::connect((*node).left);
            Solution::connect((*node).right);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // need to test, but leetcode does not suppot Rust to solve this problem
    }
}
