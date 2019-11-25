fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let s: Vec<char> = secret.chars().collect();
        let g: Vec<char> = guess.chars().collect();
        let mut amount_a = 0;
        let mut amount_b = 0;
        let mut map = HashMap::new();
        for i in 0..s.len() {
            if g[i] == s[i] {
                amount_a += 1;
            } else {
                let count = map.entry(s[i]).or_insert(0);
                *count += 1;
            }
        }
        for i in 0..s.len() {
            if g[i] != s[i] && map.get(&g[i]) != None {
                amount_b += 1;
                let count = map.entry(g[i]).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    map.remove(&g[i]);
                }
            }
        }
        format!("{0}A{1}B", amount_a, amount_b)
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
2.1 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
