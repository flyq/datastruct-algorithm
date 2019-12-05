fn main() {
    println!("Hello, world!");
}

struct MinStack {
    v: Vec<i32>,
    minindex:  usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        let s = MinStack {
            v: vec![],
            minindex: 0,
        };
        s
    }
    
    fn push(&mut self, x: i32) {
        self.v.push(x);
        if self.v[self.minindex] > x {
            self.minindex = self.v.len() - 1;
        }
    }
    
    fn pop(&mut self) {

            
        self.v.pop();
        if self.v.len() == 0 {
            self.minindex = 0;
        } else {
            if self.minindex == self.v.len() {
                let mut min = self.v[0];
                self.minindex = 0;
                for i in 0..self.v.len() {
                    if self.v[i] < min {
                        min = self.v[i];
                        self.minindex = i;
                    }
                }
            }
        }
    }
    
    fn top(&self) -> i32 {
        let len = self.v.len();
        if len != 0 {
            return self.v[len-1];
        }
        0
    }
    
    fn get_min(&self) -> i32 {
        if self.v.len() == 0 {
            return 0;
        }
        self.v[self.minindex]
    }
}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

/*
执行结果：
通过
显示详情
执行用时 :
4 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
5.4 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
