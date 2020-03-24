fn main() {
    println!("Hello, world!");
}


pub struct Solution {}

use std::cmp::max;
impl Solution {
    pub fn massage(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;
        
        for i in &nums {
            let temp = max(a+i,b);
            a = b;
            b = temp;
        }

        b
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
0 ms
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
