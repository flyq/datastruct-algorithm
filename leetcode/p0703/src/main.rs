fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::BinaryHeap;
struct KthLargest {
    priority_queue: BinaryHeap<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut k = KthLargest {
            priority_queue: BinaryHeap::with_capacity(k as usize),
        };
        for n in nums {
            k.add(n);
        }
        k
    }
    fn get_kth(&self) -> i32 {
        *self.priority_queue.peek().unwrap()
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.priority_queue.len() < self.priority_queue.capacity() {
            self.priority_queue.push(-val);
            return self.get_kth();
        }
        let curr_kth = self.get_kth();
        if (val <= curr_kth) {
            return curr_kth;
        }
        self.priority_queue.pop();
        self.priority_queue.push(-val);
        self.get_kth()
    }
}
