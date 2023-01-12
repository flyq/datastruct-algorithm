use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

struct MyLinkedList {
    l: VecDeque<i32>,
}

impl MyLinkedList {
    fn new() -> Self {
        Self { l: VecDeque::new() }
    }

    fn get(&self, index: i32) -> i32 {
        if self.l.len() as i32 > index {
            self.l[index as usize]
        } else {
            -1
        }
    }

    fn add_at_head(&mut self, val: i32) {
        self.l.push_front(val)
    }

    fn add_at_tail(&mut self, val: i32) {
        self.l.push_back(val)
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if self.l.len() as i32 >= index {
            self.l.insert(index as usize, val)
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if self.l.len() as i32 >= index {
            self.l.remove(index as usize);
        }
    }
}

/*
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
