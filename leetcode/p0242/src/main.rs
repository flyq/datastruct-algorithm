fn main() {
    println!("Hello, world!");
}

pub struct Solution{}
pub struct Solution1{}

use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = HashMap::new();
        let sv: Vec<char> = s.chars().collect();
        let tv: Vec<char> = t.chars().collect();
        for i in 0..sv.len() {
            let count = map.entry(sv[i]).or_insert(0);
            *count +=1;
        }
        for i in 0..tv.len() {
            if map.get(&tv[i]) == None {
                return false;
            } else {
                let count = map.entry(tv[i]).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    map.remove(&tv[i]);
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
8 ms
, 在所有 rust 提交中击败了
52.63%
的用户
内存消耗 :
2.5 MB
, 在所有 rust 提交中击败了
100.00%
的用户
 */



impl Solution1 {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = HashMap::new();
        let sv: Vec<char> = s.chars().collect();
        let tv: Vec<char> = t.chars().collect();
        for i in 0..sv.len() {
            let count = map.entry(sv[i]).or_insert(0);
            *count +=1;
        }
        for i in 0..tv.len() {
            if map.get(&tv[i]) == None || map.get(&tv[i]) == Some(&0) {
                return false;
            } else {
                let count = map.entry(tv[i]).or_insert(0);
                *count -= 1;
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
8 ms
, 在所有 rust 提交中击败了
52.63%
的用户
内存消耗 :
2.4 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
