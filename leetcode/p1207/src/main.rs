fn main() {
    println!("Hello, world!");
}
pub struct Solution{}

use std::collections::HashMap;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        for i in arr {
            let count = map1.entry(i).or_insert(0);
            *count += 1;
        }
        for value in map1.values() {
            if map2.get(value) == None {
                map2.insert(value, 1);
            } else {
                return false;
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
2 MB
, 在所有 rust 提交中击败了
100.00%
的用户
*/
