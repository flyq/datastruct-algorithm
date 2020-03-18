use std::cmp::{min, max};

fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        min(rec1[3], rec2[3]) > max(rec1[1], rec2[1]) && min(rec1[2],rec2[2]) > max(rec1[0], rec2[0])
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
2.1 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
