use std::collections::LinkedList;

fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut list = LinkedList::new();
        let mut head = head;
        while let Some(mut node) = std::mem::replace(&mut head, None) {
            list.push_back(node.val);
            head = std::mem::replace(&mut node.next, None);
        }
        while list.len() > 1 {
            if list.pop_back() != list.pop_front() {
                return false;
            }
        }
        true
    }
}
