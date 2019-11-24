fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let len = s.len();
        if len == 0 {
            return true;
        }
        let mut temp = HashMap::new();
        let sv: Vec<char> = s.chars().collect();
        let tv: Vec<char> = t.chars().collect();
        for i in 0..len {
            let a = temp.get(&sv[i]);
            if a != None {
                if *a.unwrap() != tv[i] {
                    return false;
                }
            } else {
                if temp.values().find(|&&x| x == tv[i]) != None {
                    return false;
                } else {
                    temp.insert(sv[i], tv[i]);
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
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
