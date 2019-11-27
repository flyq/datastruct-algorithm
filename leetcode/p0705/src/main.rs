fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;
pub struct MyHashSet {
    mut v: HashMap;
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        let mut v = HashMap::new();
        MyHashSet{ v }
    }
    
    fn add(&self, key: i32) {

    }
    
    fn remove(&self, key: i32) {
        
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
    
