fn main() {
    println!("Hello, world!");
    println!("{}", Solution::reverse(123));
}
pub struct Solution {}

use std::i32::MAX;
use std::i32::MIN;
    
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut res = 0;
        while x != 0 {
            let pop = x % 10;
            if res > MAX / 10 || (res == MAX / 10 && pop > 7) {
                return 0;
            }
            if res < MIN / 10 || (res == MIN / 10 && pop < -8) {
                return 0;
            }
            res = res * 10 + pop;
            x = x / 10;
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
, 在所有 Rust 提交中击败了
100.00%
的用户
内存消耗 :
2.1 MB
, 在所有 Rust 提交中击败了
100.00%
的用户
*/
