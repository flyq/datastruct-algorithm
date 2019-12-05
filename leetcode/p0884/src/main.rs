fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::VecDeque;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut stack1: VecDeque<char> = VecDeque::new();
        let mut stack2: VecDeque<char> = VecDeque::new();
        for i in s.chars() {
            if i == '#' {
                stack1.pop_back();
            } else {
                stack1.push_back(i);
            }
        }
        for i in t.chars() {
            if i == '#' {
                stack2.pop_back();
            } else {
                stack2.push_back(i);
            }
        }
        stack1 == stack2
    }
}

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
