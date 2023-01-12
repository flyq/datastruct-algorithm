use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct LRUCache {
    capacity: i32,
    data: HashMap<i32, i32>,
    lru_key: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            data: HashMap::new(),
            lru_key: Vec::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.data.get(&key) {
            Some(v) => {
                let mut index = 0;
                for (idx, k) in self.lru_key.iter().enumerate() {
                    if *k == key {
                        index = idx;
                        break;
                    }
                }
                self.lru_key.remove(index);
                self.lru_key.insert(0, key);
                return *v;
            }
            None => {
                return -1;
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.data.get(&key).is_some() {
            let mut index = 0;
            for (idx, k) in self.lru_key.iter().enumerate() {
                if *k == key {
                    index = idx;
                    break;
                }
            }
            self.lru_key.remove(index);
        } else {
            if self.data.len() == self.capacity as usize {
                let a = self.lru_key.pop().unwrap();
                self.data.remove(&a);
            }
        }

        self.data.insert(key, value);
        self.lru_key.insert(0, key);
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
