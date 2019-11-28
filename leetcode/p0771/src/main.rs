fn main() {
    println!("Hello, world!");
}

pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut map = HashMap::new();
        let jv: Vec<char> = j.chars().collect();
        let sv: Vec<char> = s.chars().collect();
        let mut res = 0;
        for i in 0..jv.len() {
            map.insert(jv[i], 1);
        }
        for i in 0..sv.len() {
            if map.get(&sv[i]) != None {
                res += 1;
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
