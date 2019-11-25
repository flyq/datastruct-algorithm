fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut map = HashMap::new();
        for i in 0..t.len() {
            let count = map.entry(t[i]).or_insert(0);
            *count += 1;
        }

        for i in 0..s.len() {
            let count = map.entry(s[i]).or_insert(0);
            *count -= 1;
            if *count == 0 {
                map.remove(&s[i]);
            }
        }
        for i in map.keys() {
            return *i;
        }
        ' '
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
