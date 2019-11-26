fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let len = candies.len();
        let set: HashSet<i32> = HashSet::from_iter(candies);
        if set.len() > len/2 {
            return (len/2) as i32
        }
        set.len() as i32
    }
}

/*
执行结果：
通过
显示详情
执行用时 :
32 ms
, 在所有 rust 提交中击败了
100.00%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
