fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let p: Vec<char> = pattern.chars().collect();
        let s: Vec<&str> = str.as_str().split(' ').collect();
        let lenp = p.len();
        let lens = s.len();
        if lenp != lens {
            return false;
        }
        let mut map = HashMap::new();
        let mut amap = HashMap::new();
        for i in 0..lens {
            if map.get(&s[i]) == None  {
                if amap.get(&p[i]) != None {
                    return false;
                } else {
                    map.insert(s[i],p[i]);
                    amap.insert(p[i],s[i]);
                }
            } else  {
                if *map.get(&s[i]).unwrap() != p[i] {
                    return false;
                }
            }
        }
        true                    
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
