fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut min = 2001;
        let mut map = HashMap::new();
        let mut res: Vec<String> = vec![];
        for i in 0..list1.len() {
            map.insert(&list1[i],i);
        }
        for i in 0..list2.len() {
            if map.get(&list2[i]) != None && map[&list2[i]]+i<min {
                min = map[&list2[i]]+i;
            }
        }
        for i in 0..list2.len() {
            if map.get(&list2[i]) != None && map[&list2[i]]+i == min {
                res.push(list2[i].clone());
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
16 ms
, 在所有 rust 提交中击败了
71.43%
的用户
内存消耗 :
2.3 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
