fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::VecDeque;
struct MyStack {
    v:VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack {
            v: VecDeque::new(),
        }
    }
    
    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.v.push_back(x);
    }
    
    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        let len = self.v.len();
        for i in 0..len-1 {
            let temp = self.v.pop_front().unwrap();
            self.v.push_back(temp);
        }
        self.v.pop_front().unwrap()
    }
    
    /** Get the top element. */
    fn top(&self) -> i32 {
        let len = self.v.len();
        *self.v.get(len-1).unwrap()
    }
    
    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.v.is_empty()
    }
}

/*
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
