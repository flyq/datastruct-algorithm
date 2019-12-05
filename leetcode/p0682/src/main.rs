fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::VecDeque;
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();
        let mut res = 0;
        for i in 0..ops.len() {
            match ops[i].as_str() {
                "+" => {
                    let temp = stack.pop_back().unwrap();
                    let now = stack.back().unwrap() + temp;
                    stack.push_back(temp);
                    stack.push_back(now);
                    res += now;
                },
                "D" => {
                    let now = stack.back().unwrap() * 2;
                    stack.push_back(now);
                    res += now;
                }
                "C" => {
                    res -= stack.pop_back().unwrap();
                    
                }
                _ => {
                    let now = ops[i].parse().unwrap();
                    stack.push_back(now);
                    res += now;
                }
            }
        }
        res                    
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
