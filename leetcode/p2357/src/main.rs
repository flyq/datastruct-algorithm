use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<_>>();
        let mut len = set.len() as i32;

        if set.contains(&0) {
            len -= 1;
        }
        len
    }
}
