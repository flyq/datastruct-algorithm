fn main() {
    println!("Hello, world!");
    let a ="leetcode".to_string();
    println!("{}", Solution::first_uniq_char(a));
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();

        let v: Vec<char> = s.chars().collect();

         for i in 0..v.len() {
             let count = map.entry(v[i]).or_insert(0);
             *count += 1;
         }
        
        for i in 0..v.len() {
            if map[&v[i]] == 1 {
                return i as i32;
            }
        }
        -1      
    }
}
   
/*
执行结果：
通过
显示详情
执行用时 :
36 ms
, 在所有 rust 提交中击败了
26.67%
的用户
内存消耗 :
2.2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
