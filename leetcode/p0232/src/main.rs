fn main() {
    println!("Hello, world!");
}

use std::collections::VecDeque;
struct MyQueue {
    v: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue{
            v: VecDeque::new(),
        }
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.v.push_back(x)
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        let mut temp;
        let mut vt: VecDeque<i32> = VecDeque::new();
        let res;
        
        for i in 0..self.v.len()-1 {
            temp = self.v.pop_back();
            vt.push_back(temp.unwrap());
        }
        res = self.v.pop_back().unwrap();
        for i in 0..vt.len() {
            temp = vt.pop_back();
            self.v.push_back(temp.unwrap());
        }
        res
    }
    
    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        let mut temp = Some(0);
        let mut vt: VecDeque<i32> = VecDeque::new();
        let mut res;

        
        for i in 0..self.v.len() {
            temp = self.v.pop_back();
            vt.push_back(temp.unwrap());
        }
        res = temp.unwrap();
        for i in 0..vt.len() {
            temp = vt.pop_back();
            self.v.push_back(temp.unwrap());
        }
        res
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.v.is_empty()
    }
}

/*
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */


/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
