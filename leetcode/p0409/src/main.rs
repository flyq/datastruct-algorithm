fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let a: Vec<char> = s.chars().collect();
        let mut map = HashMap::new();
        let mut res = 0;
        let mut temp = 0;
        for i in 0..a.len() {
            let count = map.entry(a[i]).or_insert(0);
            *count += 1;
        }
        for v in map.values() {
            if v % 2 == 0 {
                res += v;
            } else {
                if temp == 0 {
                    res += 1;
                    temp = 1;
                }
                res += v - 1;
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
    
