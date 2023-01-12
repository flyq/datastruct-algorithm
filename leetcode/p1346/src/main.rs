use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map = HashSet::new();
        for i in arr {
            if map.get(&i).is_some() {
                return true;
            }
            map.insert(i * 2);
            if i & 1 == 0 {
                map.insert(i >> 1);
            }
        }
        false
    }
}
